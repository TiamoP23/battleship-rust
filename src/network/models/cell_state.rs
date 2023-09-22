use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum CellState {
    #[serde(rename = "x")]
    Damaged,
    #[serde(rename = "X")]
    Destroyed,
    #[serde(rename = ".")]
    Empty,
    #[serde(rename = "O")]
    Ship,
    #[serde(rename = "")]
    Unknown,
}
