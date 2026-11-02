use std::{
   env,
   path::PathBuf,
};

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

pub(super) fn default_database_path() -> Option<PathBuf> {
   match env::consts::OS {
      "windows" => {
         absolute_env_path("LOCALAPPDATA").map(|directory| directory.join("Meridian/meridian.db"))
      },
      "linux" => {
         absolute_env_path("XDG_STATE_HOME")
            .or_else(|| absolute_env_path("HOME").map(|home| home.join(".local/state")))
            .map(|directory| directory.join("meridian/meridian.db"))
      },
      _ => None,
   }
}
