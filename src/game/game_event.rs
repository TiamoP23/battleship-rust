use crate::network::models::{Board, BoardOrBool, GameResultEvent, GameRoundEvent};

impl GameRoundEvent {
    pub fn get_self_board(&self) -> &Board {
        let self_number = self.details.get_self_number();

        &self.boards[self_number]
    }

    pub fn get_opponent_board(&self) -> &Board {
        let opponent_number = self.details.get_opponent_number();

        &self.boards[opponent_number]
    }
}

impl GameResultEvent {
    pub fn get_self_board(&self) -> Option<&Board> {
        let self_number = self.details.get_self_number();

        match self.boards[self_number] {
            BoardOrBool::Board(ref board) => Some(board),
            BoardOrBool::Bool(_) => None,
        }
    }

    pub fn get_opponent_board(&self) -> Option<&Board> {
        let opponent_number = self.details.get_opponent_number();

        match self.boards[opponent_number] {
            BoardOrBool::Board(ref board) => Some(board),
            BoardOrBool::Bool(_) => None,
        }
    }
}
