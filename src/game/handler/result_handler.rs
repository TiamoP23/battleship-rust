use log::{error, info};

use crate::network::models::GameResultEvent;

pub fn result_handler(event: GameResultEvent) {
    let self_player = event.details.get_self();
    let opponent = event.details.get_opponent();

    let game_id = &event.details.id;
    let rounds = event.details.log.len();

    if self_player.score > opponent.score {
        info!("Won game {} after {} rounds!", game_id, rounds);
    } else if self_player.score < opponent.score {
        info!("Lost game {} after {} rounds!", game_id, rounds);

        if let Some(last_round) = event.details.log.last() {
            if let Some(error) = &last_round.error {
                error!("Game {} ended with error: {}", game_id, error);
                return;
            }
        }
    } else {
        info!("Tied game {} after {} rounds!", game_id, rounds);
    }
}
