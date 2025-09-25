use crate::board::TileStates;
use crate::player::Player;

pub struct GameState {
    player1: Player,
    // Player 2 will be the bot or the other player in multiplayer mode
    player2: Player,
    // true for player1's turn, false for player2's turn
    turn: bool,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            player1: Player::new(),
            player2: Player::new(),
            turn: true,
        }
    }

    pub fn switch_turn(&mut self) {
        self.turn = !self.turn;
    }

    pub fn get_current_player_board(&self) -> Vec<Vec<TileStates>> {
        if self.turn {
            self.player1.get_board()
        } else {
            self.player2.get_board()
        }
    }

    pub fn update_board(&mut self, row: u8, col: u8) {
        if self.turn {
            self.player1.update_board(row, col);
        } else {
            self.player2.update_board(row, col);
        }
    }

    pub fn reset_game(&mut self) {
        self.player1.reset_board();
        self.player2.reset_board();
        self.turn = true;
    }

    pub fn bot_attack(&mut self) {
        if !self.turn {
            self.player2.bot_attack();
        }
    }
}
