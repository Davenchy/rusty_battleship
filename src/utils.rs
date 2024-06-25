use crate::SIZE;
use std::io::{self, Write};

pub fn get_player_input() -> (usize, usize) {
    loop {
        print!("Enter coordinates to fire: `x, y`: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let coordinates: Vec<usize> = input
            .trim()
            .split(',')
            .map(|s| s.trim().parse().expect("Invalid Input"))
            .collect();

        if coordinates.len() == 2 && coordinates[0] < SIZE && coordinates[1] < SIZE {
            return (coordinates[0], coordinates[1]);
        } else {
            println!("Invalid Input");
        }
    }
}
