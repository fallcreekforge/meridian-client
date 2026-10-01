//! Steam data access inside the customer environment.

use async_trait::async_trait;
use meridian_credential_store::{
   CredentialKey,
   CredentialStore,
   CredentialStoreError,
   SecretString,
};
use meridian_platform::{
   Platform,
   PlatformClient,
   PlatformGame,
};
use thiserror::Error;

pub mod http_client;

pub use http_client::SteamWebApiClient;

#[derive(Debug, Error)]
pub enum SteamError {
   #[error(transparent)]
   Credential(#[from] CredentialStoreError),
   #[error("Steam games are unavailable")]
   GamesUnavailable,
}

/// Narrow Steam capabilities needed by Meridian Client's local sync engine.
#[async_trait]
pub trait SteamClient {
   /// Returns the studio's games.
   ///
   /// Implementations must never log `api_key` or include it in errors.
   async fn discover_games(&self, api_key: &SecretString) -> Result<Vec<PlatformGame>, SteamError>;

   async fn sync_steam_game_financials(
      &self,
      api_key: &SecretString,
   ) -> Result<Vec<()>, SteamError>;
}

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
      let api_key = credentials.get(self.credential_key).await?;

      self.client.discover_games(&api_key).await
   }

   async fn sync_game_financials(
      &self,
      credentials: &(dyn CredentialStore + Send + Sync),
   ) -> Result<Vec<()>, Self::Error> {
      let _api_key = credentials.get(self.credential_key).await?;

      todo!()
   }
}
