use super::PlatformGameId;

/// A game discovered through an external platform.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlatformGame {
   pub platform_game_id: PlatformGameId,
   pub name:             String,
}
