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
   SteamSource,
};

use super::{
   ProtocolVersion,
   StudioId,
   SyncEngine,
};

struct TestCredentialStore {
   config: SecretString,
}

impl CredentialStore for TestCredentialStore {
   fn get(&self, key: CredentialKey) -> &SecretString {
      let _ = key;
      &self.config
   }
}

struct TestSteamClient;

// FYI - I'm still figuring out the shape of this trait.
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
   async fn sync_steam_game_financials(
      &self,
      api_key: &SecretString,
   ) -> Result<Vec<()>, SteamError> {
      let _api_key = api_key;
      Ok(Vec::new())
   }
}

#[tokio::test]
async fn sync_engine_produces_a_versioned_envelope() {
   let steam = SteamSource::new(TestSteamClient, CredentialKey::SteamFinancial);
   let envelope = SyncEngine::new(
      TestCredentialStore {
         config: SecretString::from("test-api-key"),
      },
      steam,
   )
   .sync(StudioId::new("studio_test"))
   .await
   .expect("test synchronization should succeed");

   assert_eq!(envelope.protocol_version, ProtocolVersion::V1);
   assert_eq!(envelope.studio_id.as_str(), "studio_test");
   assert_eq!(envelope.games.len(), 1);
   assert_eq!(envelope.games[0].name, "Test Game");
}
