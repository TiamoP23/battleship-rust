use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    #[serde(rename = "h")]
    Horizontal,
    #[serde(rename = "v")]
    Vertical,
}
