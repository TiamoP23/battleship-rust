use crate::network::models::{Direction, Position, Ship};

use super::with_bounds::WithBounds;

impl Ship {
    pub fn collides_with<T: WithBounds>(&self, other: &T) -> bool {
        let (self_start, self_end) = self.get_bounds();
        let (other_start, other_end) = other.get_bounds();

        self_start.x < other_end.x
            && self_end.x > other_start.x
            && self_start.y < other_end.y
            && self_end.y > other_start.y
    }

    pub fn get_occupied_fields(&self) -> Vec<Position> {
        let mut fields: Vec<Position> = vec![];

        for i in 0..self.size {
            let x = match self.direction {
                Direction::Horizontal => self.start.x + i,
                Direction::Vertical => self.start.x,
            };
            let y = match self.direction {
                Direction::Vertical => self.start.y + i,
                Direction::Horizontal => self.start.y,
            };

            fields.push(Position { x, y });
        }

        fields
    }

    pub fn get_end(&self) -> Position {
        match self.direction {
            Direction::Horizontal => Position {
                x: self.start.x + self.size - 1,
                y: self.start.y,
            },
            Direction::Vertical => Position {
                x: self.start.x,
                y: self.start.y + self.size - 1,
            },
        }
    }
}

impl WithBounds for Ship {
    fn get_bounds(&self) -> (Position, Position) {
        let (start, end) = self.start.get_bounds();

        match self.direction {
            Direction::Horizontal => (
                Position {
                    x: start.x,
                    y: start.y,
                },
                Position {
                    x: end.x + self.size - 1,
                    y: end.y,
                },
            ),
            Direction::Vertical => (
                Position {
                    x: start.x,
                    y: start.y,
                },
                Position {
                    x: end.x,
                    y: end.y + self.size - 1,
                },
            ),
        }
    }
}

impl PartialEq for Ship {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.size == other.size && self.direction == other.direction
    }
}
