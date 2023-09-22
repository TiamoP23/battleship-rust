use serde::Deserialize;

use super::{Board, BoardOrBool, GameDetails};

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum GameEvent {
    #[serde(rename = "INIT")]
    Init(GameInitEvent),
    #[serde(rename = "RESULT")]
    Result(GameResultEvent),
    #[serde(rename = "ROUND")]
    Round(GameRoundEvent),
    #[serde(rename = "SET")]
    Set(GameSetEvent),
}

#[derive(Deserialize, Debug)]
pub struct GameInitEvent {
    #[serde(flatten)]
    pub details: GameDetails,
}

#[derive(Deserialize, Debug)]
pub struct GameResultEvent {
    #[serde(flatten)]
    pub details: GameDetails,
    pub boards: [BoardOrBool; 2],
}

#[derive(Deserialize, Debug)]
pub struct GameRoundEvent {
    #[serde(flatten)]
    pub details: GameDetails,
    pub boards: [Board; 2],
}

#[derive(Deserialize, Debug)]
pub struct GameSetEvent {
    #[serde(flatten)]
    pub details: GameDetails,
}
