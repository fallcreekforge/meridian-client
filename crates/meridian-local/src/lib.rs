//! Reusable customer-side synchronization orchestration.
mod protocol;

use meridian_credential_store::CredentialStore;
use meridian_platform::{
   Platform,
   PlatformClient,
};
use meridian_types::StudioId;
pub use protocol::{
   GamePlatformV1,
   GameV1,
   ProtocolVersion,
   SyncEnvelopeV1,
};

/// Coordinates credential access, platform collection, and protocol creation.
pub struct SyncEngine<S, C> {
   credential_store: S,
   platform_client:  C,
}

impl<S, C> SyncEngine<S, C>
where
   S: CredentialStore + Send + Sync,
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
   pub async fn sync(&self, studio_id: StudioId) -> Result<SyncEnvelopeV1, C::Error> {
      let platform = protocol_platform(self.platform_client.platform())
         .expect("an invalid platform client cannot be provided to SyncEngine");

      let games = self
         .platform_client
         .discover_games(&self.credential_store)
         .await?
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

const fn protocol_platform(platform: Platform) -> Option<GamePlatformV1> {
   match platform {
      Platform::Steam => Some(GamePlatformV1::Steam),
      Platform::Unimplemented => None,
   }
}

#[cfg(test)] mod tests;
