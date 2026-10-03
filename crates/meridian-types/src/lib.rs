//! Small identifiers shared across Meridian Client's public crates.

use serde::{
   Deserialize,
   Serialize,
};

macro_rules! string_id {
   ($name:ident, $docs:literal) => {
      #[doc = $docs]
      #[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
      #[serde(transparent)]
      pub struct $name(String);

      impl $name {
         #[must_use]
         pub fn new(value: impl Into<String>) -> Self {
            Self(value.into())
         }

         #[must_use]
         pub fn as_str(&self) -> &str {
            &self.0
         }
      }
   };
}

string_id!(StudioId, "Identifies a studio in Meridian.");
string_id!(PlatformGameId, "Identifies a game on an external platform.");

#[cfg(test)] mod tests;
