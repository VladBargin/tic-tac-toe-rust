/// # Player
///
/// An enum of all possible players that a cell can hold
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    None,
    Cross,
    Nought,
}

impl Player {
    /// Updates itself to the player who has the next turn
    pub fn progress(&mut self) {
        *self = match *self {
            Player::None => Player::Cross,
            Player::Cross => Player::Nought,
            Player::Nought => Player::Cross,
        };
    }

    /// Updates itself to the player who had the turn before
    pub fn regress(&mut self) {
        *self = match *self {
            Player::None => Player::Nought,
            Player::Cross => Player::Nought,
            Player::Nought => Player::Cross,
        };
    }

    /// Returns the string representation of `player`
    pub fn to_str(&self) -> &str {
        match *self {
            Player::None => " ",
            Player::Cross => "x",
            Player::Nought => "o",
        }
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
