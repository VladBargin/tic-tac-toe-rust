use super::player::Player;

/// # GameCell
///
/// Represents a single cell in the game board
#[derive(Clone)]
pub(crate) struct GameCell {
    pub player: Player,
}

impl GameCell {
    /// Create a new `GameCell` with no player
    pub fn new() -> Self {
        Self {
            player: Player::None,
        }
    }

    /// Return `true` if occupied
    ///
    /// Otherwise return `false`
    pub fn is_occupied(&self) -> bool {
        self.player != Player::None
    }
}
