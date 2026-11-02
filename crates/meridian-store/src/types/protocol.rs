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

#[cfg(test)]
mod tests {
   use serde_json::{
      Value,
      json,
   };

   use super::{
      GameV1,
      Platform,
      PlatformGameId,
      StudioId,
      SyncEnvelopeV1,
   };

   fn sample_envelope() -> SyncEnvelopeV1 {
      SyncEnvelopeV1::new(StudioId::new("studio_test"), vec![GameV1 {
         platform:         Platform::Steam,
         platform_game_id: PlatformGameId::new("platform_game_test"),
         name:             String::from("Test Game"),
      }])
   }

   #[test]
   fn envelope_round_trips_through_json() {
      let envelope = sample_envelope();
      let encoded = serde_json::to_string(&envelope).expect("sample envelope should serialize");
      let decoded = serde_json::from_str(&encoded).expect("sample envelope should deserialize");

      assert_eq!(envelope, decoded);
      assert_eq!(
         serde_json::from_str::<Value>(&encoded).expect("sample JSON should parse"),
         json!({
            "protocol_version": "1",
            "studio_id": "studio_test",
            "games": [{
               "platform": "steam",
               "platform_game_id": "platform_game_test",
               "name": "Test Game"
            }]
         })
      );
   }

   #[test]
   fn envelope_rejects_credential_fields() {
      let mut encoded = serde_json::to_value(sample_envelope()).expect("sample should serialize");
      let Value::Object(ref mut fields) = encoded else {
         panic!("an envelope must serialize as an object");
      };
      fields.insert(String::from("credential"), json!("not-a-real-secret"));

      serde_json::from_value::<SyncEnvelopeV1>(encoded)
         .expect_err("credential fields must be rejected");
   }

   #[test]
   fn game_rejects_credential_fields() {
      let mut encoded =
         serde_json::to_value(&sample_envelope().games[0]).expect("sample game should serialize");
      let Value::Object(ref mut fields) = encoded else {
         panic!("a game must serialize as an object");
      };
      fields.insert(String::from("credential"), json!("not-a-real-secret"));

      serde_json::from_value::<GameV1>(encoded).expect_err("credential fields must be rejected");
   }

   #[test]
   fn unsupported_platform_is_rejected() {
      let mut encoded = serde_json::to_value(sample_envelope()).expect("sample should serialize");
      encoded["games"][0]["platform"] = json!("unimplemented");

      serde_json::from_value::<SyncEnvelopeV1>(encoded)
         .expect_err("unsupported platforms must be rejected");
   }
}
