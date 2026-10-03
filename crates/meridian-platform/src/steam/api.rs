use async_trait::async_trait;
use meridian_credential_store::SecretString;
use thiserror::Error;

use crate::PlatformGame;

#[derive(Debug, Error)]
pub enum SteamError {
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
