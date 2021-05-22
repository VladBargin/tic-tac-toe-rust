use crate::tic_tac_toe::game_board::GameBoard;
use text_io::try_read;

use std::io::{stdin, stdout, Write};
use termion::event::{Event, Key, MouseEvent};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;

/// Implements game state for a game of tic-tac-toe
mod tic_tac_toe;

/// Create a `GameBoard` and simulate a game of tic-tac-toe
fn main() {
    print!("Input board size (default is 3, min is 2): ");
    stdout().flush().unwrap();
    let board_size = try_read!("{}\n").unwrap_or(3).max(2);

    // Init termion
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

    let mut game_board = GameBoard::new(board_size);
    game_board.print_board(&mut stdout).unwrap();

    'game: while !game_board.game_over() {
        for c in stdin().events() {
            match c.unwrap() {
                Event::Key(Key::Char('q')) => return,
                Event::Mouse(MouseEvent::Press(_, x, y)) if x % 2 == 0 => {
                    if game_board
                        .mark_cell((x / 2 - 1) as usize, y as usize - 1)
                        .is_some()
                    {
                        game_board.print_board(&mut stdout).unwrap();
                        continue 'game;
                    }
                }
                _ => {}
            }
        }
    }

    drop(stdout);
    println!();
    game_board.print_game_over_message();
}
