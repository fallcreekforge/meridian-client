//! Platform-neutral data collection inside the customer environment.

use std::str::FromStr;

use async_trait::async_trait;
use meridian_credential_store::CredentialStore;
use meridian_types::PlatformGameId;
use serde::{
   Deserialize,
   Serialize,
};

pub mod steam;

pub use steam::{
   adapter::SteamSource,
   api::{
      SteamClient,
      SteamError,
   },
   web_api::SteamWebApiClient,
};

/// A supported external platform.
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, PartialEq)]
pub enum Platform {
   Steam,
   Unimplemented,
}

// Note, we cannot fail on this as any unrecognized Platform returns
// Ok(Platform::Unimpelemented)
impl FromStr for Platform {
   type Err = String;

   fn from_str(s: &str) -> Result<Self, Self::Err> {
      match s {
         "Steam" | "steam" => Ok(Self::Steam),
         _ => Ok(Self::Unimplemented),
      }
   }
}

/// A game discovered through an external platform.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlatformGame {
   pub platform_game_id: PlatformGameId,
   pub name:             String,
}

/// Capabilities local synchronization requires from a platform integration.
#[async_trait]
pub trait PlatformClient {
   type Error;

   fn platform(&self) -> Platform;

   async fn discover_games(
      &self,
      credentials: &(dyn CredentialStore + Send + Sync),
   ) -> Result<Vec<PlatformGame>, Self::Error>;

   async fn sync_game_financials(
      &self,
      credentials: &(dyn CredentialStore + Send + Sync),
   ) -> Result<Vec<()>, Self::Error>;
}
