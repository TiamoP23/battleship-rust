use crate::network::models::Position;

pub trait WithBounds {
    fn get_bounds(&self) -> (Position, Position);
}
