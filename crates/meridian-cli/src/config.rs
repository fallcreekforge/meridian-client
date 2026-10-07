use std::{
   collections::HashMap,
   path::PathBuf,
};

use serde::{
   Deserialize,
   Serialize,
};

// TODO: Platform Enum & Game Shape
#[rustfmt::skip]
/// Expected JSON:
///
/// ```json
/// {
///   "platforms": [
///     {
///       "steam": {
///         "allowedGames": ["game-id"]
///       }
///     }
///   ],
///   "secretFilePath": "/path/to/secrets.json"
/// }
/// ```
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
   pub platforms: Vec<HashMap<String, PlatformConfig>>,
   pub secret_file_path: PathBuf,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlatformConfig {
   pub allowed_games: Vec<String>,
}
