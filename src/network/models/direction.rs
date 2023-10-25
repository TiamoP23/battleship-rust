use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Direction {
    #[serde(rename = "h")]
    Horizontal,
    #[serde(rename = "v")]
    Vertical,
}
