//! Reusable customer-side synchronization orchestration.

use std::sync::Arc;

use meridian_credential_store::CredentialStore;
use meridian_platform::{
   Producer,
   ProducerError,
};
use thiserror::Error;
use tokio::sync::Notify;

/// Coordinates credential access, platform collection, and protocol creation.
/// Responsible for the orchestration and scheduling for Producer polling.
#[expect(dead_code)]
pub struct SyncEngine {
   credential_store: Arc<dyn CredentialStore>,
   producers:        Vec<Box<dyn Producer>>,
   outbox_notify:    Arc<Notify>,
}

#[bon::bon]
impl SyncEngine {
   /// Creates an engine with required credentials and producers, initializing
   /// its notification handle internally.
   #[builder(start_fn = builder)]
   #[must_use]
   pub fn new(
      credential_store: Arc<dyn CredentialStore>,
      producers: Vec<Box<dyn Producer>>,
   ) -> Self {
      Self {
         credential_store,
         producers,
         outbox_notify: Arc::new(Notify::new()),
      }
   }

   /// Collects and normalizes local data into the public protocol.
   pub async fn run(self) -> Result<(), SyncEngineError> {
      for producer in &self.producers {
         producer.poll(self.credential_store.as_ref()).await?;
      }

      Ok(())
   }
}

#[derive(Debug, Error)]
pub enum SyncEngineError {
   #[error(transparent)]
   Producer(#[from] ProducerError),
}

#[cfg(test)] mod tests;
