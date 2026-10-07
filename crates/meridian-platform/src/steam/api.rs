use async_trait::async_trait;
use meridian_credential_store::SecretString;
use thiserror::Error;

use crate::PlatformGame;

#[derive(Debug, Error)]
pub enum SteamError {
   #[error("Unrecognized game listed in configuration file")]
   UnrecognizedGame,
}

/// Narrow Steam capabilities needed by Meridian Client's local sync engine.
#[async_trait]
pub trait SteamClient: Send + Sync {
   /// Returns the studio's games.
   ///
   /// Implementations must never log `api_key` or include it in errors.
   async fn query_configured_game_ids(
      &self,
      api_key: &SecretString,
   ) -> Result<Vec<PlatformGame>, SteamError>;

   async fn query_game_finances(&self, api_key: &SecretString) -> Result<Vec<()>, SteamError>;
}
