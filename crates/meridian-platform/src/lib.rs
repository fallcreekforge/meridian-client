//! Platform-neutral data collection inside the customer environment.

use async_trait::async_trait;
use meridian_credential_store::CredentialStore;
pub use meridian_store::{
   Platform,
   PlatformGame,
   PlatformGameId,
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
