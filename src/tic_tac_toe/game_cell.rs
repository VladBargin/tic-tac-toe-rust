use crate::tic_tac_toe::player::{player_to_string, Player};

#[derive(Clone)]
pub(crate) struct GameCell {
    player: Player,
}

impl GameCell {
    pub fn new() -> Self {
        Self {
            player: Player::None,
        }
    }

    pub fn is_occupied(&self) -> bool {
        match self.player {
            Player::None => false,
            _ => true,
        }
    }

    pub fn get_display_string(&self) -> &str {
        player_to_string(&self.player)
    }

    pub fn set_player(&mut self, player: &Player) {
        self.player = (*player).clone();
    }

    pub fn get_player(&self) -> &Player {
        &self.player
    }
}
