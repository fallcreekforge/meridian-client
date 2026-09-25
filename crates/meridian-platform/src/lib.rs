//! Platform-neutral data collection inside the customer environment.

use async_trait::async_trait;
use meridian_credential_store::CredentialStore;
use meridian_types::PlatformGameId;

/// A supported external platform.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Platform {
   Steam,
}

/// A game discovered through an external platform.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlatformGame {
   pub platform_game_id: PlatformGameId,
   pub name:             String,
}

/// Capabilities local synchronization requires from a platform integration.
#[async_trait]
pub trait PlatformClient {
   type Error;

   fn platform(&self) -> Platform;

   async fn discover_games(
      &self,
      credentials: &(dyn CredentialStore + Send + Sync),
   ) -> Result<Vec<PlatformGame>, Self::Error>;
}
