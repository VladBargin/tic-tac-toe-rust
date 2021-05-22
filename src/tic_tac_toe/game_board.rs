use super::game_cell::GameCell;
use super::player::Player;

/// # GameBoard
///
/// Stores game state.
/// Allows you to print it and progress it.
pub struct GameBoard {
    board_size: usize,
    board: Vec<Vec<GameCell>>,
    current_player: Player,
}

impl GameBoard {
    /// Create a new game state with a square game board with size `board_size` filled with `Player::None` and current_player set to `Player::Cross`
    pub fn new(board_size: usize) -> Self {
        Self {
            board_size,
            board: vec![vec![GameCell::new(); board_size]; board_size],
            current_player: Player::Cross,
        }
    }

    /// Check if game state is terminal
    pub fn game_over(&self) -> bool {
        let player_in_main_diagonal = self.board[0][0].player;
        let player_in_minor_diagonal = self.board[self.board_size - 1][0].player;

        let mut condition_satisfied_for_main_diagonal = true;
        let mut condition_satisfied_for_minor_diagonal = true;

        for i in 1..self.board_size {
            if self.board[i][i].player != player_in_main_diagonal {
                condition_satisfied_for_main_diagonal = false;
            }

            if self.board[self.board_size - i - 1][i].player != player_in_minor_diagonal {
                condition_satisfied_for_minor_diagonal = false;
            }
        }

        if condition_satisfied_for_main_diagonal {
            match player_in_main_diagonal {
                Player::None => (),
                _ => return true,
            }
        } else if condition_satisfied_for_minor_diagonal {
            match player_in_minor_diagonal {
                Player::None => (),
                _ => return true,
            }
        }

        for i in 0..self.board_size {
            let player_in_column = self.board[0][i].player;
            let player_in_row = self.board[i][0].player;

            let mut condition_satisfied_for_column = true;
            let mut condition_satisfied_for_row = true;

            for j in 1..self.board_size {
                if self.board[j][i].player != player_in_column {
                    condition_satisfied_for_column = false
                }

                if self.board[i][j].player != player_in_row {
                    condition_satisfied_for_row = false
                }
            }

            if condition_satisfied_for_row {
                match player_in_row {
                    Player::None => (),
                    _ => return true,
                }
            } else if condition_satisfied_for_column {
                match player_in_column {
                    Player::None => (),
                    _ => return true,
                }
            }
        }

        false
    }

    /// Set's `current_player` to player who has the next turn
    fn progress_player(&mut self) {
        self.current_player.progress();
    }

    /// Set's `current_player` to player who had the previous turn
    fn regress_player(&mut self) {
        self.current_player.regress();
    }

    /// Return `Option<()>` and set the cell at `(column, row)` to `current_player` if it is a valid position and is currently unoccupied
    ///
    /// Otherwise return `None`
    pub fn mark_cell(&mut self, column: usize, row: usize) -> Option<()> {
        if row < self.board_size
            && column < self.board_size
            && !self.board[row][column].is_occupied()
        {
            self.board[row][column].player = self.current_player;
            self.progress_player();
            Some(())
        } else {
            None
        }
    }

    pub fn print_board(&self) {
        print!("   ");
        for column in 0..self.board_size {
            print!("{} ", column);
        }
        println!();

        for row in 0..self.board_size {
            print!("{} |", row);
            for column in 0..self.board_size {
                print!("{}|", self.board[row][column].player.to_str());
            }
            println!();
        }
    }

    /// Print who's turn it is
    pub fn print_current_player_message(&self) {
        match self.current_player {
            Player::Cross => println!("Turn of crosses!"),
            Player::Nought => println!("Turn of noughts!"),
            _ => print!(""),
        }
    }

    /// Print who won the game
    pub fn print_game_over_message(&mut self) {
        self.regress_player();
        match self.current_player {
            Player::Cross => println!("Crosses win!"),
            Player::Nought => println!("Noughts win!"),
            _ => print!(""),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::tic_tac_toe::game_board::GameBoard;
    use crate::tic_tac_toe::player::Player;

    #[test]
    fn test_mark_cell() {
        let mut game_board = GameBoard::new(2);
        assert_eq!(game_board.board[0][0].is_occupied(), false);
        assert_eq!(game_board.current_player, Player::Cross);

        assert_eq!(game_board.mark_cell(0, 0), Some(()));
        assert_eq!(game_board.board[0][0].is_occupied(), true);
        assert_eq!(game_board.board[0][0].player, Player::Cross);
        assert_eq!(game_board.current_player, Player::Nought);

        assert_eq!(game_board.mark_cell(0, 0), None);
        assert_eq!(game_board.board[0][0].is_occupied(), true);
        assert_eq!(game_board.board[0][0].player, Player::Cross);
        assert_eq!(game_board.current_player, Player::Nought);

        assert_eq!(game_board.mark_cell(2, 0), None);
        assert_eq!(game_board.mark_cell(0, 2), None);

        assert_eq!(game_board.mark_cell(1, 1), Some(()));
        assert_eq!(game_board.board[1][1].is_occupied(), true);
        assert_eq!(game_board.board[1][1].player, Player::Nought);
        assert_eq!(game_board.current_player, Player::Cross);
    }

    #[test]
    fn test_1() {
        let mut game_board = GameBoard::new(2);
        game_board.mark_cell(0, 0);
        game_board.mark_cell(1, 0);
        game_board.mark_cell(1, 0);
        assert_eq!(game_board.game_over(), false);

        game_board.mark_cell(0, 1);
        assert_eq!(game_board.game_over(), true);

        game_board.mark_cell(1, 1);
        assert_eq!(game_board.game_over(), true);
    }

    #[test]
    fn test_2() {
        let mut game_board = GameBoard::new(3);
        game_board.mark_cell(0, 0);
        game_board.mark_cell(1, 0);
        game_board.mark_cell(1, 0);
        game_board.mark_cell(2, 0);
        assert_eq!(game_board.game_over(), false);

        game_board.mark_cell(1, 1);
        game_board.mark_cell(1, 2);
        assert_eq!(game_board.game_over(), false);

        game_board.mark_cell(0, 1);
        game_board.mark_cell(0, 2);
        game_board.mark_cell(0, 2);
        game_board.mark_cell(2, 1);
        assert_eq!(game_board.game_over(), true);
    }

    #[test]
    fn test_3() {
        let mut game_board = GameBoard::new(3);
        game_board.mark_cell(0, 0);
        game_board.mark_cell(0, 0);
        game_board.mark_cell(1, 0);
        game_board.mark_cell(1, 1);
        assert_eq!(game_board.game_over(), false);

        game_board.mark_cell(2, 0);
        game_board.mark_cell(2, 0);
        game_board.mark_cell(2, 2);
        assert_eq!(game_board.game_over(), true);
    }

    #[test]
    fn test_4() {
        let mut game_board = GameBoard::new(4);
        game_board.mark_cell(0, 3);
        game_board.mark_cell(0, 0);
        game_board.mark_cell(1, 2);
        game_board.mark_cell(2, 2);
        game_board.mark_cell(2, 2);
        game_board.mark_cell(2, 1);
        assert_eq!(game_board.game_over(), false);

        game_board.mark_cell(1, 1);
        game_board.mark_cell(1, 1);
        game_board.mark_cell(3, 0);
        assert_eq!(game_board.game_over(), true);
    }
}
