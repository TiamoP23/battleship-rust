use rand::{distributions::Standard, prelude::Distribution, Rng};

use crate::network::models::Direction;

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0..2) {
            0 => Direction::Horizontal,
            1 => Direction::Vertical,
            _ => unreachable!(),
        }
    }
}
