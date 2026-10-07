use async_trait::async_trait;
use meridian_credential_store::{
   CredentialKey,
   CredentialStore,
};

use super::api::SteamClient;
use crate::{
   Platform,
   Producer,
   ProducerError,
};

/// Connects a typed Steam client to the platform-neutral collection contract.
pub struct SteamSource {
   client: Box<dyn SteamClient>,
}

impl SteamSource {
   #[must_use]
   pub fn new(client: Box<dyn SteamClient>) -> Self {
      Self { client }
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
