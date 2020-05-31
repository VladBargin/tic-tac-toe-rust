use crate::tic_tac_toe::game_cell::GameCell;
use crate::tic_tac_toe::player::{get_next_player, get_previous_player, Player};

pub struct GameBoard {
    board_size: usize,
    board: Vec<Vec<GameCell>>,
    current_player: Player,
}

impl GameBoard {
    pub fn new(board_size: usize) -> Self {
        Self {
            board_size,
            board: vec![vec![GameCell::new(); board_size]; board_size],
            current_player: Player::Cross,
        }
    }

    pub fn game_over(&self) -> bool {
        let player_in_main_diagonal = self.board[0][0].get_player();
        let player_in_minor_diagonal = self.board[self.board_size - 1][0].get_player();

        let mut condition_satisfied_for_main_diagonal = true;
        let mut condition_satisfied_for_minor_diagonal = true;

        for i in 1..self.board_size {
            if *self.board[i][i].get_player() != *player_in_main_diagonal {
                condition_satisfied_for_main_diagonal = false;
            }

            if self.board[self.board_size - i - 1][i]
                .get_player()
                .ne(player_in_minor_diagonal)
            {
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
            let player_in_column = self.board[i][0].get_player();
            let player_in_row = self.board[0][i].get_player();

            let mut condition_satisfied_for_column = true;
            let mut condition_satisfied_for_row = true;

            for j in 1..self.board_size {
                if self.board[i][j].get_player().ne(player_in_column) {
                    condition_satisfied_for_column = false
                }

                if self.board[j][i].get_player().ne(player_in_row) {
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

    fn progress_player(&mut self) {
        self.current_player = get_next_player(&self.current_player)
    }

    fn regress_player(&mut self) {
        self.current_player = get_previous_player(&self.current_player)
    }

    pub fn mark_cell(&mut self, column: usize, row: usize) -> Option<()> {
        if row < self.board_size
            && column < self.board_size
            && !self.board[column][row].is_occupied()
        {
            self.board[column][row].set_player(&self.current_player);
            self.progress_player();
            Some(())
        } else {
            println!("Invalid turn!");
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
                print!("{}|", self.board[row][column].get_display_string());
            }
            println!();
        }
    }

    pub fn print_current_player_message(&self) {
        match self.current_player {
            Player::Cross => println!("Turn of crosses!"),
            Player::Nought => println!("Turn of noughts!"),
            _ => print!(""),
        }
    }

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
    #[test]
    fn test_new() {}

    #[test]
    fn test_mark_cell() {}

    #[test]
    fn test_game_over() {}
}
