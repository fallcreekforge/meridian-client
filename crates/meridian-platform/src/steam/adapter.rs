use async_trait::async_trait;
use meridian_credential_store::{
   CredentialKey,
   CredentialStore,
};

use super::api::SteamClient;
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

impl SteamSource {
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

   async fn poll(&self, credentials: &dyn CredentialStore) -> Result<(), ProducerError> {
      let publisher = credentials.get(CredentialKey::SteamPublisher);
      let _financial = credentials.get(CredentialKey::SteamFinancial);

      let _games = self.client.discover_games(publisher).await?;
      todo!()
   }
}
