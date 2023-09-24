use rand::distributions::{Distribution, Standard};
use rand::Rng;

use crate::network::models::Position;

use super::with_bounds::WithBounds;

impl Distribution<Position> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Position {
        Position {
            x: rng.gen_range(0..10),
            y: rng.gen_range(0..10),
        }
    }
}

impl WithBounds for Position {
    fn get_bounds(&self) -> (Position, Position) {
        let start = Position {
            x: self.x - 1,
            y: self.y - 1,
        };
        let end = Position {
            x: self.x + 1,
            y: self.y + 1,
        };

        (start, end)
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
