use crate::tic_tac_toe::game_cell::GameCell;

pub struct GameBoard {
    board: Vec<Vec<GameCell>>, // current_player: ???
}

impl GameBoard {
    pub fn new(board_size: i32) -> Self {
        Self { board: Vec::new() }
    }

    pub fn game_over(&self) -> bool {
        false
    }

    pub fn mark_cell(&self) {}

    pub fn print_board(&self) {}
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn test_new() {}

    #[test]
    fn test_mark_cell() {}

    #[test]
    fn test_game_over() {}
}
