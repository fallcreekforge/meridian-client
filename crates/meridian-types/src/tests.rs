use super::{
   PlatformGameId,
   StudioId,
};

#[test]
fn identifiers_preserve_their_values() {
   let studio_id = StudioId::new("studio_test");
   let platform_game_id = PlatformGameId::new("game_test");

   assert_eq!(studio_id.as_str(), "studio_test");
   assert_eq!(platform_game_id.as_str(), "game_test");
}
