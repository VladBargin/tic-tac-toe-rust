use crate::tic_tac_toe::player::{player_to_string, Player};

/// # GameCell
///
/// Represents a single cell in the game board
#[derive(Clone)]
pub(crate) struct GameCell {
    player: Player,
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
        match self.player {
            Player::None => false,
            _ => true,
        }
    }

    /// Return string to be printed to represent this game cell
    pub fn get_display_string(&self) -> &str {
        player_to_string(&self.player)
    }

    /// Set's the player in this game cell
    pub fn set_player(&mut self, player: &Player) {
        self.player = (*player).clone();
    }

    /// Returns a pointer to the player in this game cell
    pub fn get_player(&self) -> &Player {
        &self.player
    }
}
