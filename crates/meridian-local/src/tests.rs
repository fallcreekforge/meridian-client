use async_trait::async_trait;
use meridian_credential_store::{
   CredentialKey,
   CredentialStore,
   ExposeSecret as _,
   SecretString,
};
use meridian_platform::{
   PlatformGame,
   PlatformGameId,
   SteamClient,
   SteamError,
};

#[expect(dead_code)]
struct TestCredentialStore {
   config: SecretString,
}

impl CredentialStore for TestCredentialStore {
   fn get(&self, key: CredentialKey) -> &SecretString {
      let _ = key;
      &self.config
   }
}

#[expect(dead_code)]
struct TestSteamClient;

#[async_trait]
impl SteamClient for TestSteamClient {
   async fn discover_games(&self, api_key: &SecretString) -> Result<Vec<PlatformGame>, SteamError> {
      assert_eq!(api_key.expose_secret(), "test-api-key");
      Ok(vec![PlatformGame {
         platform_game_id: PlatformGameId::new("game_test"),
         name:             String::from("Test Game"),
      }])
   }

   // Skeleton implementation for now
   async fn fetch_financials(&self, api_key: &SecretString) -> Result<Vec<()>, SteamError> {
      let _api_key = api_key;
      Ok(Vec::new())
   }
}
