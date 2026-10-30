use std::path::PathBuf;

use meridian_platform::Platform;
use serde::{
   Deserialize,
   Serialize,
};

/// Non-secret runtime configuration. See `sample/config.json` for the JSON
/// shape.
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Config {
   pub credentials_file: PathBuf,
   pub games:            Vec<GameConfig>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GameConfig {
   pub name:      String,
   pub platforms: Vec<Platform>,
}
