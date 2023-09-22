use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub enum PlayerSymbol {
    O,
    X,
}
