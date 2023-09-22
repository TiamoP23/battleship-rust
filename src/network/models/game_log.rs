use serde::Deserialize;

use super::Position;

#[derive(Deserialize, Debug)]
pub struct GameLog {
    pub player: String,
    #[serde(rename = "move")]
    pub game_move: Option<Position>,
}
