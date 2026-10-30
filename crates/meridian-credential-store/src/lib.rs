//! Credential storage for studio-controlled environments.
//!
//! Implementations must not log secret values or include them in errors.

pub use meridian_utils::config_file::JsonFileError as CredentialStoreError;
pub use secrecy::{
   ExposeSecret,
   SecretString,
};

mod file_credential_store;
mod secret_config;

pub use file_credential_store::FileCredentialStore;

/// Identifies a credential without exposing its value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CredentialKey {
   SteamPublisher,
   SteamFinancial,
   MeridianCloud,
}

/// Retrieves credentials held inside studio-controlled infrastructure,
/// returning an owned `SecretString`.
pub trait CredentialStore: Send + Sync {
   fn get(&self, key: CredentialKey) -> &SecretString;
}
