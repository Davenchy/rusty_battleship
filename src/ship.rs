use crate::SIZE;
use rand::prelude::*;

pub enum ShipDir {
    Vert,
    Hori,
}

pub struct Ship {
    x: usize,
    y: usize,
    size: usize,
    dir: ShipDir,
}

pub struct ShipIterator<'a> {
    ship: &'a Ship,
    index: usize,
}

impl Ship {
    pub fn iter(&self) -> ShipIterator<'_> {
        ShipIterator {
            ship: self,
            index: 0,
        }
    }

    pub fn random(min_size: usize, max_size: usize) -> Self {
        let mut rng = rand::thread_rng();

        let [x, y]: [usize; 2] = [rng.gen_range(0..SIZE); 2];
        let size = rng.gen_range(min_size..=max_size);
        let padding = SIZE - size - 1;
        let dir = match rng.gen_bool(0.5) {
            true => ShipDir::Vert,
            false => ShipDir::Hori,
        };

        Self {
            x: match dir {
                ShipDir::Vert => x,
                ShipDir::Hori => x.min(padding),
            },
            y: match dir {
                ShipDir::Vert => y.min(padding),
                ShipDir::Hori => y,
            },
            dir,
            size,
        }
    }

    pub fn collide(&self, other: &Ship) -> bool {
        self.iter()
            .any(|(x, y)| other.iter().any(|(x2, y2)| x == x2 && y == y2))
    }
}

impl Iterator for ShipIterator<'_> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.ship.size {
            return None;
        }

        let val = self.index;
        self.index += 1;

        Some(match self.ship.dir {
            ShipDir::Vert => (self.ship.x, self.ship.y + val),
            ShipDir::Hori => (self.ship.x + val, self.ship.y),
        })
    }
}
