use std::path::{
   Path,
   PathBuf,
};

use meridian_platform::Platform;
use serde::{
   Deserialize,
   Serialize,
};

use super::paths::default_database_path;

/// Non-secret runtime configuration. See `sample/config.json` for the JSON
/// shape.
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Config {
   pub credentials_file: PathBuf,
   #[serde(default, skip_serializing_if = "Option::is_none")]
   pub database_path:    Option<PathBuf>,
   pub games:            Vec<GameConfig>,
}

impl Config {
   pub fn resolve_database_path(&self, config_file: &Path) -> Result<PathBuf, &'static str> {
      if let Some(path) = self.database_path.as_deref() {
         if path.as_os_str().is_empty() {
            return Err("databasePath must not be empty.");
         }
         let directory = config_file.parent().unwrap_or_else(|| Path::new("."));
         return Ok(directory.join(path));
      }

      default_database_path().ok_or("No default database location is available; set databasePath.")
   }
}

/// A title selected by name across explicit platforms, before local ID
/// resolution.
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GameConfig {
   pub name:      String,
   pub platforms: Vec<Platform>,
}

#[cfg(test)]
mod tests {
   use std::path::Path;

   use meridian_platform::Platform;
   use serde_json::json;

   use super::Config;

   #[test]
   fn games_have_names_and_explicit_platforms() {
      let value = json!({
         "credentialsFile": "/etc/meridian/secrets.json",
         "games": [
            { "name": "Example Game", "platforms": ["steam"] },
            { "name": "Another Game", "platforms": [] }
         ]
      });
      let config: Config = serde_json::from_value(value.clone()).unwrap();
      assert!(config.database_path.is_none());
      assert_eq!(config.games[0].name, "Example Game");
      assert_eq!(config.games[0].platforms, [Platform::Steam]);
      assert_eq!(config.games[1].name, "Another Game");
      assert!(config.games[1].platforms.is_empty());
      assert_eq!(serde_json::to_value(config).unwrap(), value);
   }

   #[test]
   fn database_path_round_trips_when_supplied() {
      let value = json!({
         "credentialsFile": "secrets.json",
         "databasePath": "volume/meridian.db",
         "games": []
      });
      let config: Config = serde_json::from_value(value.clone()).unwrap();
      assert_eq!(
         config.database_path.as_deref(),
         Some(Path::new("volume/meridian.db"))
      );
      assert_eq!(serde_json::to_value(config).unwrap(), value);
   }

   #[test]
   fn rejects_unknown_fields_platforms_and_missing_required_fields() {
      for value in [
         json!({"games": [], "credentialsFile": "secrets.json", "typo": true}),
         json!({"games": [{"name": "Example Game", "platforms": ["unsupported"]}], "credentialsFile": "secrets.json"}),
         json!({"games": [{"name": "Example Game", "platforms": ["steam"], "typo": true}], "credentialsFile": "secrets.json"}),
         json!({"games": [{"platforms": ["steam"]}], "credentialsFile": "secrets.json"}),
         json!({"games": [{"name": "Example Game"}], "credentialsFile": "secrets.json"}),
         json!({"credentialsFile": "secrets.json"}),
         json!({"games": []}),
      ] {
         serde_json::from_value::<Config>(value)
            .expect_err("invalid configuration must be rejected");
      }
   }

   #[test]
   fn rejects_previous_platform_first_shapes_and_secret_path_field() {
      for value in [
         json!({"platforms": {"steam": {"allowedGames": ["480"]}}, "secretFilePath": "secrets.json"}),
         json!({"platforms": [{"steam": {"allowedGames": ["480"]}}], "secretFilePath": "secrets.json"}),
         json!({"games": [], "secretFilePath": "secrets.json"}),
      ] {
         serde_json::from_value::<Config>(value)
            .expect_err("previous configuration shapes must be rejected");
      }
   }
}
