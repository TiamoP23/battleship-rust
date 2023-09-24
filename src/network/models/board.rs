use serde::Deserialize;

use super::FieldState;

#[derive(Debug)]
pub struct Board {
    pub fields: Vec<Vec<FieldState>>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum BoardOrBool {
    Board(Board),
    Bool(bool),
}

impl<'de> Deserialize<'de> for Board {
    fn deserialize<D>(deserializer: D) -> Result<Board, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let fields = Deserialize::deserialize(deserializer)?;
        Ok(Board { fields })
    }
}
