use async_trait::async_trait;
use meridian_credential_store::CredentialStore;

use super::SteamClient;
use crate::{
   Ingest,
   Platform,
   Producer,
   ProducerError,
};

/// Connects a typed Steam client to the platform-neutral collection contract.
#[expect(dead_code)]
pub struct SteamSource {
   client: Box<dyn SteamClient>,
   ingest: Box<dyn Ingest>,
}

#[bon::bon]
impl SteamSource {
   /// Creates a source with a required client and ingestion handle.
   #[builder(start_fn = builder)]
   #[must_use]
   pub fn new(client: Box<dyn SteamClient>, ingest: Box<dyn Ingest>) -> Self {
      Self { client, ingest }
   }
}

#[async_trait]
impl Producer for SteamSource {
   fn platform(&self) -> Platform {
      Platform::Steam
   }

   async fn poll(&self, _credentials: &dyn CredentialStore) -> Result<(), ProducerError> {
      todo!()
   }
}
