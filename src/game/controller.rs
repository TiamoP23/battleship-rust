use crate::network::models::GameEvent;
use crate::network::models::GameResponse;

use super::handler::{init_handler, result_handler, round_handler, set_handler};

pub fn handle_game_event(event: GameEvent) -> Option<GameResponse> {
    match event {
        GameEvent::Init(event) => {
            init_handler(event);
            None
        }
        GameEvent::Round(event) => {
            let position = round_handler(event);

            Some(GameResponse::Attack(position))
        }
        GameEvent::Result(event) => {
            result_handler(event);
            None
        }
        GameEvent::Set(event) => {
            let placement = set_handler(event);

            Some(GameResponse::PlaceShips(placement))
        }
    }
}
