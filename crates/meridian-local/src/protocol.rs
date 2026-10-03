//! The explicit, versioned contract allowed to cross into Meridian Cloud.
//!
//! Changes to these types can expand the customer-to-cloud trust boundary and
//! require security review. Platform authentication material is deliberately
//! absent from this type system.

use meridian_types::{
   PlatformGameId,
   StudioId,
};
use serde::{
   Deserialize,
   Serialize,
};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ProtocolVersion {
   #[serde(rename = "1")]
   V1,
}

/// A game approved for synchronization.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GameV1 {
   pub platform:         GamePlatformV1,
   /// The game's identifier on the external platform.
   pub platform_game_id: PlatformGameId,
   pub name:             String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GamePlatformV1 {
   Steam,
}

/// The complete payload permitted by public sync protocol version 1.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SyncEnvelopeV1 {
   pub protocol_version: ProtocolVersion,
   pub studio_id:        StudioId,
   pub games:            Vec<GameV1>,
}

impl SyncEnvelopeV1 {
   #[must_use]
   pub fn new(studio_id: StudioId, games: Vec<GameV1>) -> Self {
      Self {
         protocol_version: ProtocolVersion::V1,
         studio_id,
         games,
      }
   }
}

#[cfg(test)]
#[path = "protocol_tests.rs"]
mod tests;
