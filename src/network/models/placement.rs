use serde::Serialize;

use super::Ship;

#[derive(Debug, Clone)]
pub struct Placement {
    pub ships: Vec<Ship>,
}

impl Serialize for Placement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.ships.serialize(serializer)
    }
}
