# Rusty Battleship CLI Game

Welcome to the Battleship CLI game implemented in Rust!
This project is part of my learning journey with Rust, and it includes several
key features such as user input parsing, map rendering, and basic AI for the
computer player.

## Features

- **User Input Parsing**: The game accepts coordinates from the user to fire at
  the enemy ships.
- **Map Rendering**: The game renders the map for both the player and the
  computer, showing hits, misses, and remaining ships.
- **Basic AI**: The computer plays as the opponent with a basic algorithm to
  fire at the player's ships.

## Example Output

Below is a sample output where the player won the game:

```mathematica
Player:
       0 1 2 3 4 5 6 7 8 9 

    0  # O O O O O . O O O 
    1  # . O O O . X . O O 
    2  # O . . O . X . O O 
    3  O O . X . . X . O O 
    4  O O O X . . X . O O 
    5  O O O X O . X . . O 
    6  O O O # O . X . X . 
    7  O O O # O . X . X . 
    8  O O O O O . X . X . 
    9  O O O O O O . O . . 

Computer:
       0 1 2 3 4 5 6 7 8 9 

    0  . O O . O . O . O O 
    1  O . O O . O O . O O 
    2  O O . . O O . O O . 
    3  O O . X X X . O O O 
    4  O O O . . . O O O O 
    5  . X X X X X X X X X 
    6  O O O O O . . O O O 
    7  O O O O O O O . O O 
    8  O O . O O O O O . O 
    9  O . X X X X X X X . 

Enter coordinates to fire: `x, y`: 0,3
You Missed!
Computer Missed!
...

Enter coordinates to fire: `x, y`: 9,6
You Hit!
Computer Missed!
Player:
       0 1 2 3 4 5 6 7 8 9 

    0  # O O O O O . O O O 
    1  # . O O O . X . O O 
    2  # O . . O . X . O O 
    3  O O . X . . X . O O 
    4  O O . X . . X . O O 
    5  O O . X . . X . . O 
    6  O O . X . . X . X . 
    7  O O . X . . X . X . 
    8  O O O . O . X . X . 
    9  O O O O O O . O . . 

Computer:
       0 1 2 3 4 5 6 7 8 9 

    0  . O O . O . O . O O 
    1  O . O O . O O . O O 
    2  O O . . O O . O O . 
    3  . . . X X X . O O O 
    4  O O O . . . O O O O 
    5  . X X X X X X X X X 
    6  O . O . O . . O . X 
    7  O . O O O O O . O X 
    8  . O . O O O O O . X 
    9  O . X X X X X X X . 

You Won!
```

## Getting Started

To get started with this project, follow these steps:

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (Make sure you have Rust installed)

### Running the Game

1. **Clone the repository**:

    ```sh
    git clone https://github.com/yourusername/battleship-cli.git
    cd battleship-cli
    ```

2. **Build the project**:

    ```sh
    cargo build --release
    ```

3. **Run the game**:

    ```sh
    cargo run --release
    ```

### Playing the Game

When you run the game, you'll be prompted to enter coordinates to fire at
the computer's ships. Enter the coordinates in the format `x, y`,
where `x` and `y` are the column and row indices respectively.

## Project Structure

- `src/main.rs`: The main entry point of the application.
  Handles the game main loop and user interactions.

- `src/board.rs`: Contains the board struct and related implementations and the
  AI algorithm for the computer player.

- `src/ship.rs`: Contains the Ship struct and related implementations.

- `src/utils.rs`: Utility functions for the game, contains the play input
  parsing function.

## License

This project is licensed under the MIT License.

---

Happy coding and have fun playing Battleship!
