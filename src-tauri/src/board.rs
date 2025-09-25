use rand::prelude::*;
use serde::{Deserialize, Serialize};

const BOARD_SIZE: usize = 10;

#[derive(Clone, PartialEq, Serialize, Copy, Deserialize, Debug)]
#[serde(tag = "type", content = "value", rename_all = "PascalCase")]
pub enum TileStates {
    Empty,
    Ship(u8),
    Hit(u8),
    Miss,
    Dead(u8),
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Board {
    board: Vec<Vec<TileStates>>,
}

impl Board {
    // pub fn load_game(board: Vec<Vec<TileStates>>) -> Self {
    //     Board { board: board }
    // }

    pub fn new() -> Self {
        let mut board = vec![vec![TileStates::Empty; BOARD_SIZE]; BOARD_SIZE];

        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        let mut rng = rand::rng();

        for (idx,ship_num) in [2,3,3,4,5].into_iter().rev().enumerate() {
            let mut placed = false;
            while !placed {
                let row = rng.random_range(0..BOARD_SIZE);
                let col = rng.random_range(0..BOARD_SIZE);
                let dir = directions[rng.random_range(0..directions.len())];

                let mut can_place = true;
                for i in 0..ship_num {
                    let new_row = (row as isize + i as isize * dir.0) as usize;
                    let new_col = (col as isize + i as isize * dir.1) as usize;

                    if new_row >= BOARD_SIZE
                        || new_col >= BOARD_SIZE
                        || board[new_row][new_col] != TileStates::Empty
                    {
                        can_place = false;
                        break;
                    }
                }

                if can_place {
                    for i in 0..ship_num {
                        let new_row = (row as isize + i as isize * dir.0) as usize;
                        let new_col = (col as isize + i as isize * dir.1) as usize;
                        board[new_row][new_col] = TileStates::Ship(idx as u8);
                    }
                    placed = true;
                }
            }
        }

        Board { board }
    }

    pub fn update(&mut self, row: u8, col: u8) {
        let mut ship_num = None;
        self.board[row as usize][col as usize] = match self.board[row as usize][col as usize] {
            TileStates::Empty => TileStates::Miss,
            TileStates::Ship(x) => {
                ship_num = Some(x);
                TileStates::Hit(x)
            }
            _ => TileStates::Miss,
        };

        if let Some(num) = ship_num {
            if !self
                .board
                .iter()
                .any(|row| row.contains(&TileStates::Ship(num)))
            {
                for r in 0..BOARD_SIZE {
                    for c in 0..BOARD_SIZE {
                        if self.board[r][c] == TileStates::Hit(num) {
                            self.board[r][c] = TileStates::Dead(num);
                        }
                    }
                }
            }
        }
    }

    fn random_attack(&mut self) {
        let mut rng = rand::rng();

        let mut row = rng.random_range(0..BOARD_SIZE);
        let mut col = rng.random_range(0..BOARD_SIZE);

        while match self.board[row][col] {
            TileStates::Empty | TileStates::Ship(_) => false,
            _ => true,
        } {
            row = rng.random_range(0..BOARD_SIZE);
            col = rng.random_range(0..BOARD_SIZE);
        }

        self.update(row as u8, col as u8);
    }

    pub fn get_board(&self) -> Vec<Vec<TileStates>> {
        self.board.clone()
    }

    pub fn reset(&mut self) {
        self.board = vec![vec![TileStates::Empty; BOARD_SIZE]; BOARD_SIZE];
        self.board = Board::new().board;
    }

    pub fn bot_attack(&mut self) {
        let pos = self
            .board
            .iter()
            .flatten()
            .position(|&tile| matches!(tile, TileStates::Hit(_)));

        if let Some(pos) = pos {
            let row = pos / BOARD_SIZE;
            let col = pos % BOARD_SIZE;
            if let Some((row, col)) = self.bot_scan(row, col) {
                self.update(row as u8, col as u8);
            } else {
                self.random_attack();
            }
        } else {
            self.random_attack();
        }
    }

    fn bot_scan(&self, row: usize, col: usize) -> Option<(usize, usize)> {
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        let mut pref_dir: Vec<(isize, isize)> = Vec::new();

        for &(dx, dy) in &directions {
            let new_row = row as isize + dx;
            let new_col = col as isize + dy;

            if new_row >= 0
                && new_row < BOARD_SIZE as isize
                && new_col >= 0
                && new_col < BOARD_SIZE as isize
            {
                let tile = self.board[new_row as usize][new_col as usize];

                if matches!(tile, TileStates::Hit(_)) {
                    pref_dir.push((dx, dy));
                    pref_dir.push((-dx, -dy)); 
                }


            }
        }

        if pref_dir.is_empty() {
            for &(dx, dy) in &directions {
                let new_row = row as isize + dx;
                let new_col = col as isize + dy;

                if new_row >= 0
                    && new_row < BOARD_SIZE as isize
                    && new_col >= 0 && new_col < BOARD_SIZE as isize && matches!(
                        self.board[new_row as usize][new_col as usize],
                        TileStates::Empty | TileStates::Ship(_)
                    ) {
                    return Some((new_row as usize, new_col as usize));
                }
            }
        }

        'dir_loop: for (dx, dy) in pref_dir {
            let mut new_row = row as isize + dx;
            let mut new_col = col as isize + dy;

            while {
                if new_row < 0
                    || new_row >= BOARD_SIZE as isize
                    || new_col < 0
                    || new_col >= BOARD_SIZE as isize
                {
                    continue 'dir_loop;
                }

                matches!(self.board[new_row as usize][new_col as usize],
                TileStates::Hit(_))
            } {
                new_row += dx;
                new_col += dy;

                
            }
            if matches!(
                self.board[new_row as usize][new_col as usize],
                TileStates::Empty | TileStates::Ship(_)
            ) {
                return Some((new_row as usize, new_col as usize));
            }
        }
        None
    }
}
