use crate::tic_tac_toe::game_board::GameBoard;
use std::cmp::max;
use std::io::{stdout, Write};
use text_io::try_read;

/// Implements game state for a game of tic-tac-toe
mod tic_tac_toe;

/// Flushes `stdout`
fn flush() {
    let _ = stdout().flush();
}

/// Create a `GameBoard` and simulate a game of tic-tac-toe
fn main() {
    print!("Input board size (default is 3, min is 2): ");
    flush();
    let board_size = match try_read!("{}\n") {
        Ok(t) => max(2, t),
        Err(_e) => 3,
    };

    let mut game_board = GameBoard::new(board_size);
    game_board.print_board();
    while !game_board.game_over() {
        println!();
        game_board.print_current_player_message();

        print!("Input column (x): ");
        flush();
        let column: usize = try_read!().unwrap_or(board_size);

        print!("Input row    (y): ");
        flush();
        let row: usize = try_read!().unwrap_or(board_size);

        match game_board.mark_cell(column, row) {
            Some(()) => game_board.print_board(),
            None => println!("Invalid turn!"),
        }
    }

    println!();
    game_board.print_game_over_message();
}
