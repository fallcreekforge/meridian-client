//! Steam data access inside the customer environment.

use std::marker::Sync;

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

pub mod web_api_client;

pub use web_api_client::SteamHttpClient;

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
}

/// Connects a typed Steam client to the platform-neutral collection contract.
pub struct SteamSource<C>
where
   C: Send + Sync,
{
   client:         C,
   credential_key: CredentialKey,
}

impl<C> SteamSource<C>
where
   C: Send + Sync,
{
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
   C: SteamClient + Send + Sync,
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
}
