//! Reusable customer-side synchronization orchestration.

use meridian_credential_store::CredentialStore;
use meridian_platform::{
   Platform,
   PlatformClient,
};
use meridian_sync_protocol::{
   GamePlatformV1,
   GameV1,
   SyncEnvelopeV1,
};
use meridian_types::StudioId;
/// Coordinates credential access, platform collection, and protocol creation.
pub struct SyncEngine<S, C> {
   credential_store: S,
   platform_client:  C,
}

impl<S, C> SyncEngine<S, C>
where
   S: CredentialStore,
   C: PlatformClient,
{
   #[must_use]
   pub fn new(credential_store: S, platform_client: C) -> Self {
      Self {
         credential_store,
         platform_client,
      }
   }

   /// Collects and normalizes local data into the public protocol.
   pub fn sync(&self, studio_id: StudioId) -> Result<SyncEnvelopeV1, C::Error> {
      let platform = protocol_platform(self.platform_client.platform());
      let games = self
         .platform_client
         .discover_games(&self.credential_store)?
         .into_iter()
         .map(|game| {
            GameV1 {
               platform,
               platform_game_id: game.platform_game_id,
               name: game.name,
            }
         })
         .collect();

      Ok(SyncEnvelopeV1::new(studio_id, games))
   }
}

const fn protocol_platform(platform: Platform) -> GamePlatformV1 {
   match platform {
      Platform::Steam => GamePlatformV1::Steam,
   }
}

#[cfg(test)]
mod tests {
   use meridian_credential_store::{
      CredentialKey,
      CredentialStore,
      CredentialStoreError,
      ExposeSecret,
      SecretString,
   };
   use meridian_platform::PlatformGame;
   use meridian_steam::{
      SteamClient,
      SteamError,
      SteamSource,
   };
   use meridian_sync_protocol::ProtocolVersion;
   use meridian_types::{
      PlatformGameId,
      StudioId,
   };

   use super::SyncEngine;

   struct TestCredentialStore;

   impl CredentialStore for TestCredentialStore {
      fn get(&self, key: &CredentialKey) -> Result<SecretString, CredentialStoreError> {
         assert_eq!(key, &CredentialKey::SteamIPartnerFinancialsService);
         Ok(SecretString::from("test-api-key"))
      }
   }

   struct TestSteamClient;

   impl SteamClient for TestSteamClient {
      fn discover_games(&self, api_key: &SecretString) -> Result<Vec<PlatformGame>, SteamError> {
         assert_eq!(api_key.expose_secret(), "test-api-key");
         Ok(vec![PlatformGame {
            platform_game_id: PlatformGameId::new("game_test"),
            name:             String::from("Test Game"),
         }])
      }
   }

   #[test]
   fn sync_engine_produces_a_versioned_envelope() {
      let steam = SteamSource::new(
         TestSteamClient,
         CredentialKey::SteamIPartnerFinancialsService,
      );
      let envelope = SyncEngine::new(TestCredentialStore, steam)
         .sync(StudioId::new("studio_test"))
         .expect("test synchronization should succeed");

      assert_eq!(envelope.protocol_version, ProtocolVersion::V1);
      assert_eq!(envelope.studio_id.as_str(), "studio_test");
      assert_eq!(envelope.games.len(), 1);
      assert_eq!(envelope.games[0].name, "Test Game");
   }
}
