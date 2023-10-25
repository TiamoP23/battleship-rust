use serde::Serialize;

use super::{Direction, Position};

#[derive(Serialize, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Ship {
    pub start: Position,
    pub size: i8,
    pub direction: Direction,
}
