#[derive(Debug)]

/// # Player
///
/// An enum of all possible players that a cell can hold
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

/// Get's the `Player` who has the turn after `player`
pub fn get_next_player(player: &Player) -> Player {
    match *player {
        Player::None => Player::Cross,
        Player::Cross => Player::Nought,
        Player::Nought => Player::Cross,
    }
}

/// Get's the `Player` who had the turn before `player`
pub fn get_previous_player(player: &Player) -> Player {
    match *player {
        Player::None => Player::Nought,
        Player::Cross => Player::Nought,
        Player::Nought => Player::Cross,
    }
}

/// Returns the string representation of `player`
pub fn player_to_string(player: &Player) -> &str {
    match *player {
        Player::None => " ",
        Player::Cross => "x",
        Player::Nought => "o",
    }
}

#[cfg(test)]
pub mod tests {
    use crate::tic_tac_toe::player::Player;

    #[test]
    fn test_eq() {
        let t10 = Player::None;
        let t11 = Player::None;
        let t20 = Player::Cross;
        let t21 = Player::Cross;
        let t30 = Player::Nought;
        let t31 = Player::Nought;

        assert_eq!(t10 == t11, true);
        assert_eq!(t20 == t21, true);
        assert_eq!(t30 == t31, true);
        assert_eq!(t10 == t20, false);
        assert_eq!(t20 == t30, false);
        assert_eq!(t30 == t10, false);
    }
}
