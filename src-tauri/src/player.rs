use crate::board::{Board, TileStates};

pub struct Player {
    pub board: Board,
    ships_alive: u8,
}

impl Player {
    pub fn new() -> Self {
        Player {
            board: Board::new(),
            ships_alive: 5,
        }
    }

    pub fn update_board(&mut self, row: u8, col: u8) {
        self.board.update(row, col);
    }

    pub fn get_board(&self) -> Vec<Vec<TileStates>> {
        self.board.get_board()
    }

    pub fn reset_board(&mut self) {
        self.board.reset();
        self.ships_alive = 5; 
    }

    // pub fn is_dead(self) -> bool {
    //     self.ships_alive == 0
    // }

    pub fn bot_attack(&mut self) {
        self.board.bot_attack();
    }

}
