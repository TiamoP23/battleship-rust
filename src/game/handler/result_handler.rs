use log::{error, info};
use sqlx::query;

use crate::{application::database::DB_POOL, network::models::GameResultEvent};

#[derive(sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
enum GameResult {
    Win,
    Loss,
    Tie,
}

pub async fn result_handler(event: GameResultEvent) {
    let self_player = event.details.get_self();
    let opponent = event.details.get_opponent();

    let game_id = &event.details.id;
    let rounds = event.details.log.len();

    let result = if self_player.score > opponent.score {
        GameResult::Win
    } else if self_player.score < opponent.score {
        GameResult::Loss
    } else {
        GameResult::Tie
    };

    match result {
        GameResult::Win => {
            info!("Won game {} after {} rounds!", game_id, rounds);
        }
        GameResult::Loss => {
            info!("Lost game {} after {} rounds!", game_id, rounds);

            if let Some(last_round) = event.details.log.last() {
                if let Some(error) = &last_round.error {
                    error!("Game {} ended with error: {}", game_id, error);
                    return;
                }
            }
        }
        GameResult::Tie => {
            info!("Tied game {} after {} rounds!", game_id, rounds);
        }
    }

    let pool = DB_POOL.get().unwrap();

    query!(
        "UPDATE game SET ended_at = NOW(), result = $1 WHERE game_id = $2",
        result as GameResult,
        game_id
    )
    .execute(pool)
    .await
    .expect("Failed to update game in database");
}
