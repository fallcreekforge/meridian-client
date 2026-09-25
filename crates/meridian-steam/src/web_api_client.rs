use async_trait::async_trait;
use reqwest::Client;

use crate::SteamClient;

pub struct SteamHttpClient {
   pub http: Client,
}

impl SteamHttpClient {
   #[must_use]
   pub fn new() -> Self {
      Self {
         http: Client::new(),
      }
   }
}

#[async_trait]
impl SteamClient for SteamHttpClient {
   async fn discover_games(
      &self,
      api_key: &meridian_credential_store::SecretString,
   ) -> Result<Vec<meridian_platform::PlatformGame>, crate::SteamError> {
      let _ = api_key;
      todo!()
   }
}
