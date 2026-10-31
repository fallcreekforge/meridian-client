use std::{
   env,
   path::{
      Path,
      PathBuf,
   },
};

use super::Config;

fn absolute_env_path(variable: &str) -> Option<PathBuf> {
   env::var_os(variable)
      .map(PathBuf::from)
      .filter(|directory| directory.is_absolute())
}

pub fn default_config_path() -> Option<PathBuf> {
   match env::consts::OS {
      "windows" => {
         absolute_env_path("LOCALAPPDATA").map(|directory| directory.join("Meridian/config.json"))
      },
      "linux" => {
         absolute_env_path("XDG_CONFIG_HOME")
            .or_else(|| absolute_env_path("HOME").map(|home| home.join(".config")))
            .map(|directory| directory.join("meridian/config.json"))
      },
      _ => None,
   }
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

      let path = match env::consts::OS {
         "windows" => {
            absolute_env_path("LOCALAPPDATA").map(|directory| directory.join("Meridian/state.db"))
         },
         "linux" => {
            absolute_env_path("XDG_STATE_HOME")
               .or_else(|| absolute_env_path("HOME").map(|home| home.join(".local/state")))
               .map(|directory| directory.join("meridian/state.db"))
         },
         _ => None,
      };
      path.ok_or("No default database location is available; set databasePath.")
   }
}
