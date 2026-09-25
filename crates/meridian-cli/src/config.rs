use serde::{
   Deserialize,
   Serialize,
};

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
   pub secret_file_path: String,
}
