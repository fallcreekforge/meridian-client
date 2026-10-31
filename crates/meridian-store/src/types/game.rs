use super::PlatformGameId;

/// A game discovered through an external platform.
///
/// The client or producer supplies the platform context. This is distinct from
/// a name-based configuration entry and from the versioned outbound payload.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlatformGame {
   pub platform_game_id: PlatformGameId,
   pub name:             String,
}
