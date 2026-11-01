use std::fs;

use sqlx::{
   Connection as _,
   error::DatabaseError,
   migrate::MigrateError,
};

use super::support::TestDatabase;
use crate::{
   Outbox,
   OutboxError,
   migrations::BUNDLED,
};

const INITIAL: &str = include_str!("../../tests/fixtures/database.sql");
const REBUILD: &str = include_str!("../../tests/fixtures/rebuild.sql");

#[tokio::test]
async fn bundled_migrations_initialize_history() {
   let database = TestDatabase::new();
   let outbox = Outbox::open(&database.path).await.unwrap();
   let versions: Vec<i64> =
      sqlx::query_scalar("SELECT version FROM _sqlx_migrations ORDER BY version")
         .fetch_all(&outbox.pool)
         .await
         .unwrap();
   let expected: Vec<i64> = BUNDLED
      .iter()
      .filter(|migration| !migration.migration_type.is_down_migration())
      .map(|migration| migration.version)
      .collect();
   assert_eq!(versions, expected);
   outbox.pool.close().await;
}

#[tokio::test]
async fn reopening_skips_applied_migrations_and_ignores_checksum_file() {
   let database = TestDatabase::new();
   let migrator = database.migrator(&[("1_initial.sql", INITIAL)]).await;
   let outbox = Outbox::open_with_migrator(&database.path, &migrator)
      .await
      .unwrap();
   sqlx::query("INSERT INTO test_parent VALUES (1, 'synthetic parent')")
      .execute(&outbox.pool)
      .await
      .unwrap();
   outbox.pool.close().await;
   let reopened = Outbox::open_with_migrator(&database.path, &migrator)
      .await
      .unwrap();
   let counts: (i64, i64) = sqlx::query_as(
      "SELECT (SELECT count(*) FROM _sqlx_migrations), (SELECT count(*) FROM test_parent)",
   )
   .fetch_one(&reopened.pool)
   .await
   .unwrap();
   assert_eq!(counts, (1, 1));
   reopened.pool.close().await;
}

#[tokio::test]
async fn table_rebuild_preserves_referencing_rows_and_runtime_foreign_keys() {
   let database = TestDatabase::new();
   let original = database.migrator(&[("1_initial.sql", INITIAL)]).await;
   let outbox = Outbox::open_with_migrator(&database.path, &original)
      .await
      .unwrap();
   sqlx::raw_sql(
      "INSERT INTO test_parent VALUES (1, 'synthetic parent'); INSERT INTO test_child VALUES (1, \
       1);",
   )
   .execute(&outbox.pool)
   .await
   .unwrap();
   outbox.pool.close().await;

   let upgrade = database.migrator(&[("2_rebuild.sql", REBUILD)]).await;
   let reopened = Outbox::open_with_migrator(&database.path, &upgrade)
      .await
      .unwrap();
   let counts: (i64, i64, i64) = sqlx::query_as(
      "SELECT (SELECT count(*) FROM _sqlx_migrations), (SELECT count(*) FROM test_parent), \
       (SELECT count(*) FROM test_child)",
   )
   .fetch_one(&reopened.pool)
   .await
   .unwrap();
   assert_eq!(counts, (2, 1, 1));
   let error = sqlx::query("INSERT INTO test_child VALUES (2, 99)")
      .execute(&reopened.pool)
      .await
      .expect_err("runtime foreign keys must be enabled");
   assert!(
      error
         .as_database_error()
         .is_some_and(DatabaseError::is_foreign_key_violation)
   );
   reopened.pool.close().await;
}

#[tokio::test]
async fn failed_batch_rolls_back_schema_and_history_and_releases_the_database() {
   let database = TestDatabase::new();
   let valid = database.migrator(&[("1_initial.sql", INITIAL)]).await;
   let invalid = database
      .migrator(&[(
         "2_invalid.sql",
         "CREATE TABLE partial (id INTEGER); INVALID SYNTHETIC SQL;",
      )])
      .await;
   let result = Outbox::open_with_migrator(&database.path, &invalid).await;
   assert!(matches!(result, Err(OutboxError::Migration(_))));
   let mut connection = database.connection().await;
   let count: i64 =
      sqlx::query_scalar("SELECT count(*) FROM sqlite_schema WHERE name NOT GLOB 'sqlite_*'")
         .fetch_one(&mut connection)
         .await
         .unwrap();
   assert_eq!(count, 0);
   connection.close().await.unwrap();
   let outbox = Outbox::open_with_migrator(&database.path, &valid)
      .await
      .unwrap();
   outbox.pool.close().await;
}

#[tokio::test]
async fn foreign_key_failure_rolls_back_the_pending_migration() {
   let database = TestDatabase::new();
   let original = database.migrator(&[("1_initial.sql", INITIAL)]).await;
   let outbox = Outbox::open_with_migrator(&database.path, &original)
      .await
      .unwrap();
   outbox.pool.close().await;
   let invalid = database
      .migrator(&[("2_orphan.sql", "INSERT INTO test_child VALUES (1, 99);")])
      .await;
   let result = Outbox::open_with_migrator(&database.path, &invalid).await;
   assert!(matches!(result, Err(OutboxError::ForeignKeyViolation)));
   let reopened = Outbox::open_with_migrator(&database.path, &original)
      .await
      .unwrap();
   let counts: (i64, i64) = sqlx::query_as(
      "SELECT (SELECT count(*) FROM _sqlx_migrations), (SELECT count(*) FROM test_child)",
   )
   .fetch_one(&reopened.pool)
   .await
   .unwrap();
   assert_eq!(counts, (1, 0));
   reopened.pool.close().await;
}

#[tokio::test]
async fn populated_database_without_history_is_rejected_without_changes() {
   let database = TestDatabase::new();
   let mut connection = database.connection().await;
   sqlx::raw_sql(INITIAL)
      .execute(&mut connection)
      .await
      .unwrap();
   sqlx::query("INSERT INTO test_parent VALUES (1, 'synthetic parent')")
      .execute(&mut connection)
      .await
      .unwrap();
   connection.close().await.unwrap();
   assert!(matches!(
      Outbox::open(&database.path).await,
      Err(OutboxError::UnversionedDatabase)
   ));
   let mut connection = database.connection().await;
   let state: (i64, bool) = sqlx::query_as(
      "SELECT (SELECT count(*) FROM test_parent), EXISTS (SELECT 1 FROM sqlite_schema WHERE name \
       = '_sqlx_migrations')",
   )
   .fetch_one(&mut connection)
   .await
   .unwrap();
   assert_eq!(state, (1, false));
   connection.close().await.unwrap();
}

#[tokio::test]
async fn modified_applied_migration_is_rejected() {
   let database = TestDatabase::new();
   let original = database.migrator(&[("1_initial.sql", INITIAL)]).await;
   let outbox = Outbox::open_with_migrator(&database.path, &original)
      .await
      .unwrap();
   outbox.pool.close().await;
   let changed = format!("{INITIAL}\n-- synthetic history edit\n");
   let modified = database.migrator(&[("1_initial.sql", &changed)]).await;
   assert!(matches!(
      Outbox::open_with_migrator(&database.path, &modified).await,
      Err(OutboxError::Migration(MigrateError::VersionMismatch(1)))
   ));
   let reopened = Outbox::open_with_migrator(&database.path, &original)
      .await
      .unwrap();
   reopened.pool.close().await;
}

#[tokio::test]
async fn missing_applied_migration_is_rejected() {
   let database = TestDatabase::new();
   let original = database.migrator(&[("1_initial.sql", INITIAL)]).await;
   let outbox = Outbox::open_with_migrator(&database.path, &original)
      .await
      .unwrap();
   outbox.pool.close().await;
   fs::remove_file(database.directory.join("migrations/1_initial.sql")).unwrap();
   let incomplete = database.migrator(&[("2_rebuild.sql", REBUILD)]).await;
   assert!(matches!(
      Outbox::open_with_migrator(&database.path, &incomplete).await,
      Err(OutboxError::Migration(MigrateError::VersionMissing(1)))
   ));
}

#[tokio::test]
async fn nontransactional_migration_is_rejected() {
   let database = TestDatabase::new();
   let migrator = database
      .migrator(&[(
         "1_unwrapped.sql",
         "-- no-transaction\nCREATE TABLE unwrapped (id INTEGER);",
      )])
      .await;
   assert!(matches!(
      Outbox::open_with_migrator(&database.path, &migrator).await,
      Err(OutboxError::NonTransactionalMigration(1))
   ));
}
