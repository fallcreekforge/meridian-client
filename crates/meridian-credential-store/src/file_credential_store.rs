//! File-backed credential storage.

use std::{
   fs::File,
   io::BufReader,
   path::PathBuf,
};

use secrecy::SecretString;
use serde_json::from_reader;

use crate::{
   CredentialKey,
   CredentialStore,
   CredentialStoreError,
   secret_config::SecretConfig,
};

/// Credentials loaded from a customer-controlled configuration file.
pub struct FileCredentialStore {
   config: SecretConfig,
}

impl FileCredentialStore {
   /// Creates a credential store from values already protected as secrets.
   pub fn load_from_file(secret_file_path: PathBuf) -> Result<Self, CredentialStoreError> {
      println!("Opening secret file: {}", secret_file_path.display());

      if !secret_file_path
         .extension()
         .and_then(|ext| ext.to_str())
         .is_some_and(|ext| ext.eq_ignore_ascii_case("json"))
      {
         println!("The secret file should be a JSON file... Exiting!");
         return Err(CredentialStoreError::IncorrectFileType);
      }

      let file = File::open(secret_file_path)?;
      let reader = BufReader::new(file);
      let config: SecretConfig = from_reader(reader)?;

      Ok(Self { config })
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
