use async_trait::async_trait;
use meridian_credential_store::{
   CredentialKey,
   CredentialStore,
};

use super::api::{
   SteamClient,
   SteamError,
};
use crate::{
   Platform,
   PlatformClient,
   PlatformGame,
};

/// Connects a typed Steam client to the platform-neutral collection contract.
pub struct SteamSource<C> {
   client:         C,
   credential_key: CredentialKey,
}

impl<C> SteamSource<C> {
   #[must_use]
   pub fn new(client: C, credential_key: CredentialKey) -> Self {
      Self {
         client,
         credential_key,
      }
   }
}

#[async_trait]
impl<C> PlatformClient for SteamSource<C>
where
   C: SteamClient + Sync,
{
   type Error = SteamError;

   fn platform(&self) -> Platform {
      Platform::Steam
   }

   async fn discover_games(
      &self,
      credentials: &(dyn CredentialStore + Send + Sync),
   ) -> Result<Vec<PlatformGame>, Self::Error> {
      let api_key = credentials.get(self.credential_key);

      self.client.discover_games(api_key).await
   }

   async fn sync_game_financials(
      &self,
      credentials: &(dyn CredentialStore + Send + Sync),
   ) -> Result<Vec<()>, Self::Error> {
      let _api_key = credentials.get(self.credential_key);

      todo!()
   }
}
