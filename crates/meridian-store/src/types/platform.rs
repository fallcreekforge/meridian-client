use serde::{
   Deserialize,
   Serialize,
};

/// A supported external platform.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Platform {
   Steam,
}
