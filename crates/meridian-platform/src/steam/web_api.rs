use reqwest::Client;

use super::SteamClient;

pub struct SteamWebApiClient {
   pub http_client: Client,
}

// TODO: move http client to top level shared resource
#[bon::bon]
impl SteamWebApiClient {
   #[builder(start_fn = builder)]
   #[must_use]
   pub fn new(http_client: Client) -> Self {
      Self { http_client }
   }
}

impl SteamClient for SteamWebApiClient {}
