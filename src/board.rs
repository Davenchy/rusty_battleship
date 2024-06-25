use std::ops::RangeInclusive;

use rand::Rng;

use crate::SIZE;

use crate::ship::Ship;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CellState {
    Empty,
    Ship,
    Miss,
    Hit,
}

#[derive(Clone)]
pub struct Board {
    map: [[CellState; SIZE]; SIZE],
}

#[derive(Debug, PartialEq)]
pub enum FireResult {
    Miss,
    Hit,
    OutOfRangeError,
}

impl Board {
    pub fn new() -> Self {
        Self {
            map: [[CellState::Empty; SIZE]; SIZE],
        }
    }

    pub fn generate_map(&mut self, mut count: usize, range: RangeInclusive<usize>) {
        let mut ships: Vec<Ship> = Vec::new();

        loop {
            if count == 0 {
                break;
            }

            let ship = Ship::random(*range.start(), *range.end());

            if !ships.iter().any(|s| s.collide(&ship)) {
                count -= 1;
                ships.push(ship);
            }
        }

        for ship in ships {
            for (x, y) in ship.iter() {
                self.map[x][y] = CellState::Ship;
            }
        }
    }

    #[must_use]
    pub fn fire(&mut self, r: usize, c: usize) -> FireResult {
        if c >= SIZE || r >= SIZE {
            return FireResult::OutOfRangeError;
        }

        self.map[r][c] = match self.map[r][c] {
            CellState::Empty => CellState::Miss,
            CellState::Ship => CellState::Hit,
            CellState::Miss => CellState::Miss,
            CellState::Hit => CellState::Hit,
        };

        if let CellState::Hit = self.map[r][c] {
            FireResult::Hit
        } else {
            FireResult::Miss
        }
    }

    pub fn random_fire(&mut self) -> FireResult {
        let mut rng = rand::thread_rng();
        loop {
            let [x, y]: [usize; 2] = [rng.gen_range(0..SIZE); 2];
            if self.map[x][y].is_empty() {
                return self.fire(x, y);
            }
        }
    }

    pub fn ai_fire(&mut self) -> FireResult {
        for r in 0..SIZE {
            for c in 0..SIZE {
                if self.map[r][c] == CellState::Hit {
                    let mut options: Vec<(usize, usize)> = Vec::new();

                    if r != 0 {
                        options.push((r - 1, c));
                    }
                    if r < SIZE - 1 {
                        options.push((r + 1, c));
                    }
                    if c != 0 {
                        options.push((r, c - 1));
                    }
                    if c < SIZE - 1 {
                        options.push((r, c + 1));
                    }

                    if let Some((r, c)) = options.iter().find(|(r, c)| self.map[*r][*c].is_empty())
                    {
                        return self.fire(*r, *c);
                    }
                }
            }
        }

        self.random_fire()
    }

    pub fn is_game_over(&self) -> bool {
        !self
            .map
            .iter()
            .any(|row| row.iter().any(|cell| *cell == CellState::Ship))
    }

    pub fn display(&self, label: &str, hide_ships: bool) {
        println!("{}:", label);

        print!("       ");
        for i in 0..SIZE {
            print!("{i} ");
        }
        println!("\n");

        for (j, row) in self.map.iter().enumerate() {
            print!("    {j}  ");
            for cell in row {
                match cell {
                    CellState::Empty => print!("O "),
                    CellState::Ship => {
                        if hide_ships {
                            print!("O ")
                        } else {
                            print!("# ")
                        }
                    }
                    CellState::Miss => print!(". "),
                    CellState::Hit => print!("X "),
                }
            }
            println!();
        }
        println!();
    }
}

impl CellState {
    fn is_empty(&self) -> bool {
        *self == CellState::Empty || *self == CellState::Ship
    }
}
