use std::path::PathBuf;

use serde::{
   Deserialize,
   Serialize,
};

// TODO: Platform Enum & Game Shape
#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
   pub secret_file_path: PathBuf,
}
