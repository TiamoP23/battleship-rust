use crate::network::models::{GameDetails, Player};

impl GameDetails {
    pub fn get_self(&self) -> &Player {
        self.players
            .iter()
            .find(|player| player.id == self.self_id)
            .unwrap()
    }

    pub fn get_opponent(&self) -> &Player {
        self.players
            .iter()
            .find(|player| player.id != self.self_id)
            .unwrap()
    }

    pub fn get_self_number(&self) -> usize {
        self.players
            .iter()
            .position(|player| player.id == self.self_id)
            .unwrap()
    }

    pub fn get_opponent_number(&self) -> usize {
        self.players
            .iter()
            .position(|player| player.id != self.self_id)
            .unwrap()
    }
}
