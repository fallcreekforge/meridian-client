//! SQLx-backed SQLite persistence and shared persisted contracts for Meridian
//! Client.

use std::path::Path;

use sqlx::{
   SqlitePool,
   sqlite::SqlitePoolOptions,
};
use thiserror::Error;

pub mod dispatch;
pub mod ingest;
pub mod types;

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
#[expect(dead_code)]
pub struct Outbox {
   // combined_collector: CombinedCollector,
   pool: SqlitePool,
}

impl Outbox {
   pub async fn open(path: impl AsRef<Path>) -> Result<Self, OutboxError> {
      let database_url = format!("sqlite://{}", path.as_ref().to_string_lossy());

      let pool = SqlitePoolOptions::new().connect(&database_url).await?;

      // TODO: enumerate migration scripts and apply to backend.

      Ok(Self { pool })
   }

   // TODO: Dispatch/Ingestion Handles
}

#[derive(Debug, Error)]
pub enum OutboxError {
   #[error("failed to connect to Meridian database at {path}")]
   FailedToConnect { path: String },

   #[error("failed to connect to Meridian database at {path}")]
   FailedToClose { path: String },

   #[error("attempted to use database before connecting")]
   NotConnected,

   #[error(transparent)]
   Sqlx(#[from] sqlx::Error),
}
