use crate::board::{Board, BLACK};

struct GameState {
    board: Board,
    current_player: i32,
    black_pass: bool,
    white_pass: bool,
    game_over: bool,
}

pub struct Game {
    board: Board,
    current_player: i32,
    black_pass: bool,
    white_pass: bool,
    game_over: bool,
    history: Vec<GameState>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: Board::new(),
            current_player: BLACK,
            black_pass: false,
            white_pass: false,
            game_over: false,
            history: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.board = Board::new();
        self.current_player = BLACK;
        self.black_pass = false;
        self.white_pass = false;
        self.game_over = false;
        self.history.clear();
    }

    fn save_state(&mut self) {
        let state = GameState {
            board: self.board.copy(),
            current_player: self.current_player,
            black_pass: self.black_pass,
            white_pass: self.white_pass,
            game_over: self.game_over,
        };
        self.history.push(state);
    }

    pub fn undo(&mut self) -> bool {
        if self.history.is_empty() {
            return false;
        }
        let state = self.history.pop().unwrap();
        self.board = state.board;
        self.current_player = state.current_player;
        self.black_pass = state.black_pass;
        self.white_pass = state.white_pass;
        self.game_over = state.game_over;
        true
    }

    pub fn make_move(&mut self, x: i32, y: i32) -> bool {
        if self.game_over {
            return false;
        }

        // Save state before making move
        self.save_state();

        if x < 0 || y < 0 {
            // Pass move
            if self.current_player == BLACK {
                self.black_pass = true;
            } else {
                self.white_pass = true;
            }

            if self.black_pass && self.white_pass {
                self.game_over = true;
            }
            self.current_player = crate::board::get_opponent(self.current_player);
            return true;
        }

        if self.board.make_move(x, y, self.current_player) {
            if self.current_player == BLACK {
                self.black_pass = false;
            } else {
                self.white_pass = false;
            }
            self.current_player = crate::board::get_opponent(self.current_player);
            return true;
        }

        // If move failed, remove the saved state
        self.history.pop();
        false
    }

    pub fn get_current_player(&self) -> i32 {
        self.current_player
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn is_game_over(&self) -> bool {
        self.game_over
    }

    pub fn get_board_state(&self) -> Vec<Vec<i32>> {
        let mut state = Vec::new();
        for y in 0..crate::board::BOARD_SIZE {
            let mut row = Vec::new();
            for x in 0..crate::board::BOARD_SIZE {
                row.push(self.board.get(x as i32, y as i32));
            }
            state.push(row);
        }
        state
    }

    pub fn calculate_score(&self) -> (i32, i32) {
        // Returns (black_score, white_score)
        // Using area scoring: stones on board + controlled territory
        let mut black_score = 0;
        let mut white_score = 0;

        for y in 0..crate::board::BOARD_SIZE {
            for x in 0..crate::board::BOARD_SIZE {
                let cell = self.board.get(x as i32, y as i32);
                match cell {
                    crate::board::BLACK => black_score += 1,
                    crate::board::WHITE => white_score += 1,
                    crate::board::EMPTY => {
                        // Check which player controls this empty space
                        let controlled_by = self.board.get_territory_owner(x as i32, y as i32);
                        match controlled_by {
                            crate::board::BLACK => black_score += 1,
                            crate::board::WHITE => white_score += 1,
                            _ => {} // Neutral territory
                        }
                    }
                    _ => {}
                }
            }
        }

        (black_score, white_score)
    }
}

