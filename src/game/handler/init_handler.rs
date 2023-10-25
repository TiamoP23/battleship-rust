use log::info;
use sqlx::query;

use crate::{application::database::DB_POOL, network::models::GameInitEvent};

pub async fn init_handler(event: GameInitEvent) {
    let game_id = event.details.id;

    info!("Started game {}!", game_id);

    let pool = DB_POOL.get().unwrap();

    query!("INSERT INTO game (game_id) VALUES ($1)", game_id)
        .execute(pool)
        .await
        .expect("Failed to insert game into database");
}
