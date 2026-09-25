//! Credential storage for studio-controlled environments.
//!
//! Implementations must not log secret values or include them in errors.

use std::io;

use async_trait::async_trait;
pub use secrecy::{
   ExposeSecret,
   SecretString,
};
use thiserror::Error;

mod file_credential_store;
mod secret_config;

pub use file_credential_store::FileCredentialStore;

/// Identifies a credential without exposing its value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CredentialKey {
   SteamIPartnerFinancialsService,
   MeridianCloud,
}

type IoError = io::Error;
type SerdeJsonError = serde_json::Error;

#[derive(Debug, Error)]
pub enum CredentialStoreError {
   #[error("secret file type is not a json file")]
   IncorrectFileType,

   #[error("I/O error: {0}")]
   Io(#[from] IoError),

   #[error("serde_json error: {0}")]
   SerdeJson(#[from] SerdeJsonError),
}

/// Retrieves credentials held inside studio-controlled infrastructure,
/// returning an owned `SecretString`.
#[async_trait]
pub trait CredentialStore {
   async fn get(&self, key: CredentialKey) -> Result<SecretString, CredentialStoreError>;
}
