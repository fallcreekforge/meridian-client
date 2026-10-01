use async_trait::async_trait;
use reqwest::Client;

use crate::SteamClient;

pub struct SteamWebApiClient {
   pub http: Client,
}

impl SteamWebApiClient {
   #[must_use]
   pub fn new() -> Self {
      Self {
         http: Client::new(),
      }
   }
}

#[async_trait]
impl SteamClient for SteamWebApiClient {
   async fn discover_games(
      &self,
      api_key: &meridian_credential_store::SecretString,
   ) -> Result<Vec<meridian_platform::PlatformGame>, crate::SteamError> {
      let _ = api_key;
      todo!()
   }

   async fn sync_steam_game_financials(
      &self,
      api_key: &meridian_credential_store::SecretString,
   ) -> Result<Vec<()>, crate::SteamError> {
      let _ = api_key;
      todo!()
   }
}
