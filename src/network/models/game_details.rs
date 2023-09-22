use serde::Deserialize;

use super::{GameLog, Player};

#[derive(Deserialize, Debug)]
pub struct GameDetails {
    pub id: String,
    pub log: Vec<GameLog>,
    pub players: [Player; 2],
    #[serde(rename = "self")]
    pub self_id: String,
}
