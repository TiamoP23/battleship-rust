use crate::network::models::{GameSetEvent, Placement};

pub fn set_handler(_event: GameSetEvent) -> Placement {
    let placement: Placement = rand::random();

    placement
}
