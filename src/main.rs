#![allow(dead_code)]

mod board;
mod ship;
mod utils;

use board::Board;
use utils::get_player_input;

pub const SIZE: usize = 10;
pub const COUNT: usize = 4;

fn main() {
    let mut board = Board::new();
    let mut pc_board = Board::new();

    board.generate_map(COUNT, 3..=8);
    pc_board.generate_map(COUNT, 3..=8);

    loop {
        board.display("Player", false);
        pc_board.display("Computer", true);

        if pc_board.is_game_over() {
            println!("You Won!");
            break;
        }

        if board.is_game_over() {
            println!("You Loss!");
            break;
        }

        let (x, y) = get_player_input();
        match pc_board.fire(y, x) {
            board::FireResult::Hit => println!("You Hit!"),
            board::FireResult::Miss => println!("You Missed!"),
            board::FireResult::OutOfRangeError => println!("Invalid Input, Try Again!"),
        };

        match board.ai_fire() {
            board::FireResult::Hit => println!("Computer Hit!"),
            board::FireResult::Miss => println!("Computer Missed!"),
            board::FireResult::OutOfRangeError => println!("Invalid Input, Try Again!"),
        };
    }
}
