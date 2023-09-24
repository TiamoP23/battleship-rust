use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i8,
    pub y: i8,
}

impl Serialize for Position {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (self.x, self.y).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Position {
    fn deserialize<D>(deserializer: D) -> Result<Position, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let (x, y) = Deserialize::deserialize(deserializer)?;
        Ok(Position { x, y })
    }
}
