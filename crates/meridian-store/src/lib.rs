//! SQLx-backed SQLite persistence and shared persisted contracts for Meridian
//! Client.

use std::path::Path;

use sqlx::{
   SqlitePool,
   migrate::{
      MigrateError,
      Migrator,
   },
   sqlite::{
      SqliteConnectOptions,
      SqlitePoolOptions,
   },
};
use thiserror::Error;

pub mod dispatch;
pub mod ingest;
mod migrations;
pub mod types;

#[cfg(test)] mod database_tests;

// TODO: `meridian-types` should be a seperate package
// allowing for both `meridian-client` and `meridian-cloud` to share the same
// domain type definitions.
pub use dispatch::Dispatch;
pub use ingest::Ingest;
pub use types::{
   GameV1,
   Platform,
   PlatformGame,
   PlatformGameId,
   ProtocolVersion,
   StudioId,
   SyncEnvelopeV1,
};

/// `Outbox` is responsible for schema migrations
/// along with providing Ingestion and Dispatch handles.
#[cfg_attr(not(test), expect(dead_code))]
pub struct Outbox {
   // combined_collector: CombinedCollector,
   pool: SqlitePool,
}

impl Outbox {
   /// Opens an existing SQLite database and applies its bundled pending
   /// migrations.
   pub async fn open(path: impl AsRef<Path>) -> Result<Self, OutboxError> {
      Self::open_with_migrator(path.as_ref(), &migrations::BUNDLED).await
   }

   async fn open_with_migrator(path: &Path, migrator: &Migrator) -> Result<Self, OutboxError> {
      migrations::apply(path, migrator).await?;
      let options = SqliteConnectOptions::new()
         .filename(path)
         .foreign_keys(true);
      let pool = SqlitePoolOptions::new().connect_with(options).await?;

      Ok(Self { pool })
   }

   // TODO: Dispatch/Ingestion Handles
}

#[derive(Debug, Error)]
pub enum OutboxError {
   #[error(transparent)]
   Sqlx(#[from] sqlx::Error),
   #[error(transparent)]
   Migration(#[from] MigrateError),
   #[error(
      "Database has schema without SQLx migration history; use an empty database or restore a \
       versioned backup."
   )]
   UnversionedDatabase,
   #[error("Migration would leave foreign-key violations; the migration batch was rolled back.")]
   ForeignKeyViolation,
   #[error("Migration {0} disables transactions; SQLite startup migrations must be transactional.")]
   NonTransactionalMigration(i64),
}
