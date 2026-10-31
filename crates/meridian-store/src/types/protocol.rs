use serde::{
   Deserialize,
   Serialize,
};

use super::{
   Platform,
   PlatformGameId,
   StudioId,
};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ProtocolVersion {
   #[serde(rename = "1")]
   V1,
}

/// A game approved for synchronization.
///
/// This versioned allowlist remains separate from platform discovery records so
/// adding locally collected fields cannot expand the outbound payload.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GameV1 {
   pub platform:         Platform,
   /// The game's identifier on the external platform.
   pub platform_game_id: PlatformGameId,
   pub name:             String,
}

/// The complete payload permitted by public sync protocol version 1.
///
/// Changes to this type can expand the customer-to-cloud trust boundary and
/// require security review. Platform authentication material is deliberately
/// absent from this type system.
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
