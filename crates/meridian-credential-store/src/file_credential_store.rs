//! File-backed credential storage.

use std::path::Path;

use meridian_utils::config_file::load_json_file;

use crate::{
   CredentialKey,
   CredentialStore,
   JsonFileError,
   SecretString,
   secret_config::SecretConfig,
};

/// Credentials loaded from a customer-controlled configuration file.
pub struct FileCredentialStore {
   config: SecretConfig,
}

impl FileCredentialStore {
   /// Loads credentials from a customer-controlled JSON file.
   pub fn load_from_file(secret_file_path: impl AsRef<Path>) -> Result<Self, JsonFileError> {
      Ok(Self {
         config: load_json_file(secret_file_path)?,
      })
   }
}

impl CredentialStore for FileCredentialStore {
   fn get(&self, key: CredentialKey) -> &SecretString {
      // TODO: Change these to be asynchronous operations
      match key {
         CredentialKey::SteamPublisher => &self.config.steam_publisher_web_api_key,
         CredentialKey::SteamFinancial => &self.config.steam_financial_web_api_key,
         CredentialKey::MeridianCloud => &self.config.meridian_cloud_key,
      }
   }
}
