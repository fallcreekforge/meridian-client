use std::{
   env,
   fs,
   path::PathBuf,
   process,
   sync::atomic::{
      AtomicU64,
      Ordering,
   },
};

use sqlx::error::DatabaseError;

use crate::{
   Outbox,
   OutboxError,
};

static NEXT_DATABASE: AtomicU64 = AtomicU64::new(0);

struct TestDatabase {
   directory: PathBuf,
   path:      PathBuf,
}

impl TestDatabase {
   #[expect(
      clippy::create_dir,
      reason = "Fail if the isolated test directory already exists."
   )]
   fn new() -> Self {
      let directory = env::temp_dir().join(format!(
         "meridian-database-{}-{}",
         process::id(),
         NEXT_DATABASE.fetch_add(1, Ordering::Relaxed)
      ));
      fs::create_dir(&directory).unwrap();
      let path = directory.join("state %23.db");
      fs::File::options()
         .write(true)
         .create_new(true)
         .open(&path)
         .unwrap();
      Self { directory, path }
   }

   async fn open(&self) -> Outbox {
      let outbox = Outbox::open(&self.path)
         .await
         .expect("open the test database");
      sqlx::raw_sql(include_str!("../../schema.sql"))
         .execute(&outbox.pool)
         .await
         .unwrap();
      sqlx::raw_sql(include_str!("../../tests/fixtures/database.sql"))
         .execute(&outbox.pool)
         .await
         .unwrap();
      outbox
   }
}

impl Drop for TestDatabase {
   fn drop(&mut self) {
      let _ = fs::remove_dir_all(&self.directory);
   }
}

#[tokio::test]
async fn database_paths_are_used_literally() {
   let database = TestDatabase::new();
   let outbox = Outbox::open(&database.path)
      .await
      .expect("open the literal filesystem path");
   let (_, _, filename): (i64, String, String) = sqlx::query_as("PRAGMA database_list")
      .fetch_one(&outbox.pool)
      .await
      .unwrap();
   assert_eq!(
      fs::canonicalize(filename).unwrap(),
      fs::canonicalize(&database.path).unwrap()
   );
   outbox.pool.close().await;
}

#[tokio::test]
async fn committed_related_rows_survive_reopening_the_database() {
   let database = TestDatabase::new();
   let outbox = database.open().await;
   let mut transaction = outbox.pool.begin().await.unwrap();
   sqlx::query("INSERT INTO test_parent (id, name) VALUES (1, 'synthetic parent')")
      .execute(&mut *transaction)
      .await
      .unwrap();
   sqlx::query("INSERT INTO test_child (id, parent_id) VALUES (1, 1)")
      .execute(&mut *transaction)
      .await
      .unwrap();
   transaction.commit().await.unwrap();
   outbox.pool.close().await;

   let reopened = Outbox::open(&database.path).await.unwrap();
   let name: String = sqlx::query_scalar(
      "SELECT name FROM test_parent JOIN test_child ON test_parent.id = test_child.parent_id",
   )
   .fetch_one(&reopened.pool)
   .await
   .unwrap();
   assert_eq!(name, "synthetic parent");
   reopened.pool.close().await;
}

#[tokio::test]
async fn pooled_connections_enforce_foreign_keys() {
   let database = TestDatabase::new();
   let outbox = database.open().await;
   let first = outbox.pool.acquire().await.unwrap();
   let second = outbox.pool.acquire().await.unwrap();
   for mut connection in [first, second] {
      let error = sqlx::query("INSERT INTO test_child (id, parent_id) VALUES (1, 99)")
         .execute(&mut *connection)
         .await
         .expect_err("orphan rows must be rejected");
      assert!(
         error
            .as_database_error()
            .is_some_and(DatabaseError::is_foreign_key_violation)
      );
   }
   outbox.pool.close().await;
}

#[tokio::test]
async fn rollback_after_a_constraint_failure_leaves_no_partial_rows() {
   let database = TestDatabase::new();
   let outbox = database.open().await;
   let mut transaction = outbox.pool.begin().await.unwrap();
   sqlx::query("INSERT INTO test_parent (id, name) VALUES (1, 'synthetic parent')")
      .execute(&mut *transaction)
      .await
      .unwrap();
   sqlx::query("INSERT INTO test_child (id, parent_id) VALUES (1, 99)")
      .execute(&mut *transaction)
      .await
      .expect_err("the second write must fail");
   transaction.rollback().await.unwrap();

   let counts: (i64, i64) = sqlx::query_as(
      "SELECT (SELECT count(*) FROM test_parent), (SELECT count(*) FROM test_child)",
   )
   .fetch_one(&outbox.pool)
   .await
   .unwrap();
   assert_eq!(counts, (0, 0));
   outbox.pool.close().await;
}

#[tokio::test]
async fn opening_an_unavailable_database_returns_a_storage_error() {
   let database = TestDatabase::new();
   let result = Outbox::open(database.directory.join("missing/state.db")).await;
   assert!(matches!(result, Err(OutboxError::Sqlx(_))));
}
