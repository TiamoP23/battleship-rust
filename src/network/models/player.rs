use serde::Deserialize;

use super::PlayerSymbol;

#[derive(Deserialize, Debug)]
pub struct Player {
    pub id: String,
    pub score: u32,
    pub symbol: Option<PlayerSymbol>,
}
