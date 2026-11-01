use std::path::Path;

use sqlx::{
   Connection as _,
   SqliteConnection,
   migrate::Migrator,
   sqlite::SqliteConnectOptions,
};

use crate::OutboxError;

pub(crate) static BUNDLED: Migrator = sqlx::migrate!("./migrations");

pub(crate) async fn apply(path: &Path, migrator: &Migrator) -> Result<(), OutboxError> {
   for migration in migrator.iter() {
      if migration.no_tx && !migration.migration_type.is_down_migration() {
         return Err(OutboxError::NonTransactionalMigration(migration.version));
      }
   }

   // Table rebuilds need foreign keys disabled before the transaction starts.
   // This connection is closed before the runtime pool enables enforcement.
   let options = SqliteConnectOptions::new()
      .filename(path)
      .foreign_keys(false);
   let mut connection = SqliteConnection::connect_with(&options).await?;
   let result = apply_transaction(&mut connection, migrator).await;
   let closed = connection.close().await;
   result?;
   closed?;
   Ok(())
}

async fn apply_transaction(
   connection: &mut SqliteConnection,
   migrator: &Migrator,
) -> Result<(), OutboxError> {
   let mut transaction = connection.begin_with("BEGIN IMMEDIATE").await?;
   let unversioned: bool = sqlx::query_scalar(
      "SELECT EXISTS (
          SELECT 1 FROM sqlite_schema
          WHERE name NOT GLOB 'sqlite_*' AND name <> '_sqlx_migrations'
      ) AND NOT EXISTS (
          SELECT 1 FROM sqlite_schema WHERE type = 'table' AND name = '_sqlx_migrations'
      )",
   )
   .fetch_one(&mut *transaction)
   .await?;
   if unversioned {
      transaction.rollback().await?;
      return Err(OutboxError::UnversionedDatabase);
   }

   migrator.run(&mut *transaction).await?;
   if sqlx::query("PRAGMA foreign_key_check")
      .fetch_optional(&mut *transaction)
      .await?
      .is_some()
   {
      transaction.rollback().await?;
      return Err(OutboxError::ForeignKeyViolation);
   }
   transaction.commit().await?;
   Ok(())
}
