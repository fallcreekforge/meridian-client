use meridian_platform::PlatformGameId;
use serde_json::{
   Value,
   json,
};

use super::{
   GamePlatformV1,
   GameV1,
   StudioId,
   SyncEnvelopeV1,
};

fn sample_envelope() -> SyncEnvelopeV1 {
   SyncEnvelopeV1::new(StudioId::new("studio_test"), vec![GameV1 {
      platform:         GamePlatformV1::Steam,
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
