use serde::Deserialize;

use super::CellState;

#[derive(Deserialize, Debug)]
pub struct Board(pub Vec<Vec<CellState>>);

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum BoardOrBool {
    Board(Board),
    Bool(bool),
}
