pub enum Player {
    None,
    Cross,
    Nought,
}

impl Clone for Player {
    fn clone(&self) -> Player {
        match self {
            Player::None => Player::None,
            Player::Cross => Player::Cross,
            Player::Nought => Player::Nought,
        }
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        player_to_string(self) == player_to_string(other)
    }
}

pub fn get_next_player(player: &Player) -> Player {
    match *player {
        Player::None => Player::Cross,
        Player::Cross => Player::Nought,
        Player::Nought => Player::Cross,
    }
}

pub fn get_previous_player(player: &Player) -> Player {
    match *player {
        Player::None => Player::Nought,
        Player::Cross => Player::Nought,
        Player::Nought => Player::Cross,
    }
}

pub fn player_to_string(player: &Player) -> &str {
    match *player {
        Player::None => " ",
        Player::Cross => "x",
        Player::Nought => "o",
    }
}
