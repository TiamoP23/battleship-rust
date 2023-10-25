use crate::network::models::GameEvent;
use crate::network::models::GameResponse;

use super::handler::{init_handler, result_handler, round_handler, set_handler};

pub async fn handle_game_event(event: GameEvent) -> Option<GameResponse> {
    match event {
        GameEvent::Init(event) => {
            init_handler(event).await;
            None
        }
        GameEvent::Round(event) => {
            let position = round_handler(event).await;

            Some(GameResponse::Attack(position))
        }
        GameEvent::Result(event) => {
            result_handler(event).await;
            None
        }
        GameEvent::Set(event) => {
            let placement = set_handler(event).await;

            Some(GameResponse::PlaceShips(placement))
        }
    }
}
