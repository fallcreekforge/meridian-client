use serde::{
   Deserialize,
   Serialize,
};

/// Identifies a studio in Meridian.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(transparent)]
pub struct StudioId(String);

impl StudioId {
   pub fn new(value: impl Into<String>) -> Self {
      Self(value.into())
   }

   #[must_use]
   pub fn as_str(&self) -> &str {
      &self.0
   }
}

/// Identifies a game on an external platform.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(transparent)]
pub struct PlatformGameId(String);

impl PlatformGameId {
   pub fn new(value: impl Into<String>) -> Self {
      Self(value.into())
   }

   #[must_use]
   pub fn as_str(&self) -> &str {
      &self.0
   }
}
