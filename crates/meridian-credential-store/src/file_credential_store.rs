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
   secret_file_path: PathBuf,
}

impl FileCredentialStore {
   /// Creates a credential store from values already protected as secrets.
   #[must_use]
   pub fn new(secret_file_path: PathBuf) -> Self {
      Self { secret_file_path }
   }
}

impl CredentialStore for FileCredentialStore {
   fn get(&self, key: &CredentialKey) -> Result<SecretString, CredentialStoreError> {
      let path = &self.secret_file_path;

      println!("Opening secret file: {}", path.display());

      if !path
         .extension()
         .and_then(|ext| ext.to_str())
         .is_some_and(|ext| ext.eq_ignore_ascii_case("json"))
      {
         println!("The secret file should be a JSON file... Exiting!");
         return Err(CredentialStoreError::IncorrectFileType);
      }

      let file = File::open(path)?;

      let reader = BufReader::new(file);

      let config: SecretConfig = from_reader(reader)?;

      match key {
         &CredentialKey::SteamIPartnerFinancialsService => {
            Ok(SecretString::from(
               config.steam_ipartner_financials_service_key,
            ))
         },
         &CredentialKey::MeridianCloud => Ok(SecretString::from(config.meridian_cloud_key)),
      }
   }
}
