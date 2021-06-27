mod tic_tac_toe;

use crate::tic_tac_toe::game_board::GameBoard;

use std::error::Error;
use std::io::{stdin, stdout, Write};
use termion::event::{Event, Key, MouseEvent};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;

/// Create a `GameBoard` and simulate a game of tic-tac-toe
fn main() -> Result<(), Box<dyn Error>> {
    print!("Input board size (default is 3, min is 2): ");
    stdout().flush()?;

    let mut buf = String::new();
    stdin().read_line(&mut buf)?;
    let mut game_board = GameBoard::new(buf.trim().parse()?);

    // Init termion
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode()?);

    'game: loop {
        game_board.print_board(&mut stdout)?;
        if game_board.game_over() {
            break;
        }
        // Handle keyboard/mouse events
        for c in stdin().events() {
            match c.unwrap() {
                // Press `q` to quit
                Event::Key(Key::Char('q')) => return Ok(()),
                Event::Mouse(MouseEvent::Press(_, x, y)) if x % 2 == 0 => {
                    if game_board
                        .mark_cell((x / 2 - 1) as usize, y as usize - 1)
                        .is_some()
                    {
                        continue 'game;
                    }
                }
                _ => {}
            }
        }
    }

    // Deinit termion
    drop(stdout);

    println!();
    game_board.print_game_over_message();

    Ok(())
}
