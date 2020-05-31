use crate::tic_tac_toe::game_board::GameBoard;
use std::io::{stdout, Write};
use text_io::try_read;

mod tic_tac_toe;

fn main() {
    print!("Input board size (default is 3): ");
    stdout().flush();
    let board_size = match try_read!("{}\n") {
        Ok(t) => t,
        Err(e) => 3,
    };

    let mut game_board = GameBoard::new(board_size);

    game_board.print_board();
    loop {
        game_board.print_current_player_message();

        print!("Input row    (x): ");
        stdout().flush();
        let row = match try_read!() {
            Ok(t) => t,
            Err(e) => board_size,
        };

        print!("Input column (y): ");
        stdout().flush();
        let column = match try_read!() {
            Ok(t) => t,
            Err(e) => board_size,
        };

        match game_board.mark_cell(column, row) {
            Some(()) => {
                game_board.print_board();
                if game_board.game_over() {
                    game_board.print_game_over_message();
                    break;
                };
            }
            None => {}
        }
    }
}
