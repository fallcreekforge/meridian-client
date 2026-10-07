//! Platform-neutral data collection inside the customer environment.

use std::str::FromStr;

use async_trait::async_trait;
use meridian_credential_store::CredentialStore;
use serde::{
   Deserialize,
   Serialize,
};
use thiserror::Error;

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

/// Identifies a game on an external platform.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(transparent)]
pub struct PlatformGameId(String);

impl PlatformGameId {
   #[must_use]
   pub fn new(value: impl Into<String>) -> Self {
      Self(value.into())
   }

   #[must_use]
   pub fn as_str(&self) -> &str {
      &self.0
   }
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

#[derive(Debug, Error)]
pub enum ProducerError {
   #[error(transparent)]
   Steam(#[from] SteamError),
}

/// Capabilities local synchronization requires from a platform integration.
#[async_trait]
pub trait Producer: Send + Sync {
   fn platform(&self) -> Platform;

   async fn poll(&self, credentials: &dyn CredentialStore) -> Result<(), ProducerError>;
}
