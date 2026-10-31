//! SQLx-backed SQLite persistence and shared persisted contracts for Meridian
//! Client.

use std::path::Path;

use sqlx::{
   SqlitePool,
   sqlite::{
      SqliteConnectOptions,
      SqlitePoolOptions,
   },
};
use thiserror::Error;

pub mod dispatch;
pub mod ingest;
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
   /// Opens an existing SQLite database using its literal filesystem path.
   pub async fn open(path: impl AsRef<Path>) -> Result<Self, OutboxError> {
      let options = SqliteConnectOptions::new()
         .filename(path)
         .foreign_keys(true);
      let pool = SqlitePoolOptions::new().connect_with(options).await?;

      // TODO: enumerate migration scripts and apply to backend.

      Ok(Self { pool })
   }

   // TODO: Dispatch/Ingestion Handles
}

#[derive(Debug, Error)]
pub enum OutboxError {
   #[error(transparent)]
   Sqlx(#[from] sqlx::Error),
}
