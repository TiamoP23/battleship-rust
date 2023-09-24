use serde::Serialize;

use crate::network::models::{Placement, Position};

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum GameResponse {
    PlaceShips(Placement),
    Attack(Position),
}
