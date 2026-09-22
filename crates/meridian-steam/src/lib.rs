//! Steam data access inside the customer environment.

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

#[derive(Debug, Error)]
pub enum SteamError {
   #[error(transparent)]
   Credential(#[from] CredentialStoreError),
   #[error("Steam games are unavailable")]
   GamesUnavailable,
}

/// Narrow Steam capabilities needed by Meridian Client's local sync engine.
pub trait SteamClient {
   /// Returns the studio's games.
   ///
   /// Implementations must never log `api_key` or include it in errors.
   fn discover_games(&self, api_key: &SecretString) -> Result<Vec<PlatformGame>, SteamError>;
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

impl<C> PlatformClient for SteamSource<C>
where
   C: SteamClient,
{
   type Error = SteamError;

   fn platform(&self) -> Platform {
      Platform::Steam
   }

   fn discover_games(
      &self,
      credentials: &dyn CredentialStore,
   ) -> Result<Vec<PlatformGame>, Self::Error> {
      let api_key = credentials.get(&self.credential_key)?;

      self.client.discover_games(&api_key)
   }
}
