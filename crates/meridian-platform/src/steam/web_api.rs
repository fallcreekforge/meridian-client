use async_trait::async_trait;
use reqwest::Client;

use super::api::{
   SteamClient,
   SteamError,
};

pub struct SteamWebApiClient {
   pub http_client: Client,
}

// TODO: move http client to top level shared resource
impl SteamWebApiClient {
   #[must_use]
   pub fn new(http_client: Client) -> Self {
      Self { http_client }
   }
}

#[async_trait]
impl SteamClient for SteamWebApiClient {
   async fn discover_games(
      &self,
      api_key: &meridian_credential_store::SecretString,
   ) -> Result<Vec<crate::PlatformGame>, SteamError> {
      let _ = api_key;
      todo!()
   }

   async fn fetch_financials(
      &self,
      api_key: &meridian_credential_store::SecretString,
   ) -> Result<Vec<()>, SteamError> {
      let _ = api_key;
      todo!()
   }
}
