// Imports: WebAssembly bindings, serialization, and data structures
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashSet, VecDeque};

// Constant: Standard Go board size (19x19)
const BOARD_SIZE: usize = 19;

// Enum: Stone types (Empty, Black, White) - exported to JavaScript
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[wasm_bindgen]
pub enum Stone {
    Empty,
    Black,
    White,
}

// Struct: Board position (row, col) with helper methods
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    // Create: New position
    fn new(row: usize, col: usize) -> Self {
        Position { row, col }
    }

    // Neighbors: Get adjacent positions (up, down, left, right)
    fn neighbors(&self) -> Vec<Position> {
        let mut neighbors = Vec::new();
        if self.row > 0 {
            neighbors.push(Position::new(self.row - 1, self.col));
        }
        if self.row < BOARD_SIZE - 1 {
            neighbors.push(Position::new(self.row + 1, self.col));
        }
        if self.col > 0 {
            neighbors.push(Position::new(self.row, self.col - 1));
        }
        if self.col < BOARD_SIZE - 1 {
            neighbors.push(Position::new(self.row, self.col + 1));
        }
        neighbors
    }
}

// Struct: Game state - board, players, captures, game status
#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct GameState {
    board: [[Stone; BOARD_SIZE]; BOARD_SIZE],
    current_player: Stone,
    previous_board: Option<[[Stone; BOARD_SIZE]; BOARD_SIZE]>, // For ko rule
    black_captured: usize,
    white_captured: usize,
    consecutive_passes: usize,
    game_over: bool,
    last_move: Option<(usize, usize)>,
}

#[wasm_bindgen]
impl GameState {
    // Constructor: Create new game with empty board, Black to play
    #[wasm_bindgen(constructor)]
    pub fn new() -> GameState {
        GameState {
            board: [[Stone::Empty; BOARD_SIZE]; BOARD_SIZE],
            current_player: Stone::Black,
            previous_board: None,
            black_captured: 0,
            white_captured: 0,
            consecutive_passes: 0,
            game_over: false,
            last_move: None,
        }
    }

    // Getters: Expose game state properties to JavaScript
    #[wasm_bindgen(getter)]
    pub fn current_player(&self) -> Stone {
        self.current_player
    }

    #[wasm_bindgen(getter)]
    pub fn game_over(&self) -> bool {
        self.game_over
    }

    #[wasm_bindgen(getter)]
    pub fn black_captured(&self) -> usize {
        self.black_captured
    }

    #[wasm_bindgen(getter)]
    pub fn white_captured(&self) -> usize {
        self.white_captured
    }

    // Board: Get entire board as JavaScript array
    pub fn get_board(&self) -> JsValue {
        let mut board_array = Vec::new();
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                let value = match self.board[row][col] {
                    Stone::Empty => 0,
                    Stone::Black => 1,
                    Stone::White => 2,
                };
                board_array.push(value);
            }
        }
        serde_wasm_bindgen::to_value(&board_array).unwrap()
    }

    // Stone: Get stone at specific position
    pub fn get_stone(&self, row: usize, col: usize) -> Stone {
        if row >= BOARD_SIZE || col >= BOARD_SIZE {
            return Stone::Empty;
        }
        self.board[row][col]
    }

    // Place: Place stone at position, handle captures, ko rule, and suicide
    pub fn place_stone(&mut self, row: usize, col: usize) -> bool {
        if self.game_over {
            return false;
        }

        if row >= BOARD_SIZE || col >= BOARD_SIZE {
            return false;
        }

        if self.board[row][col] != Stone::Empty {
            return false;
        }

        // Ko: Save board state before move
        let board_before_move = self.board.clone();

        // Place: Put stone on board
        self.board[row][col] = self.current_player;

        // Capture: Check neighbors for opponent groups to capture
        let mut captured_count = 0;
        let opponent = match self.current_player {
            Stone::Black => Stone::White,
            Stone::White => Stone::Black,
            Stone::Empty => return false,
        };

        let pos = Position::new(row, col);
        for neighbor in pos.neighbors() {
            if self.board[neighbor.row][neighbor.col] == opponent {
                let captured = self.capture_group(neighbor.row, neighbor.col);
                captured_count += captured;
            }
        }

        // Suicide: Check if placed stone has liberties (not captured)
        if self.count_liberties(row, col) == 0 {
            if captured_count == 0 {
                // Invalid: Suicide without capture
                self.board = board_before_move;
                return false;
            }
        }

        // Update: Increment captured counts
        match self.current_player {
            Stone::Black => self.black_captured += captured_count,
            Stone::White => self.white_captured += captured_count,
            Stone::Empty => {}
        }

        // Ko: Check if board state repeats (ko rule violation)
        if let Some(ref prev_board) = self.previous_board {
            if self.board == *prev_board {
                // Invalid: Ko violation - revert move
                self.board = board_before_move;
                match self.current_player {
                    Stone::Black => self.black_captured -= captured_count,
                    Stone::White => self.white_captured -= captured_count,
                    Stone::Empty => {}
                }
                return false;
            }
        }

        // Update: Save board state, record move, reset passes
        self.previous_board = Some(board_before_move);
        self.last_move = Some((row, col));
        self.consecutive_passes = 0;

        // Switch: Change to opponent's turn
        self.current_player = opponent;

        true
    }

    // Pass: Skip turn (two consecutive passes ends game)
    pub fn pass(&mut self) {
        if self.game_over {
            return;
        }

        self.consecutive_passes += 1;
        self.last_move = None;

        // If both players pass consecutively, game ends
        if self.consecutive_passes >= 2 {
            self.game_over = true;
        } else {
            // Switch player
            self.current_player = match self.current_player {
                Stone::Black => Stone::White,
                Stone::White => Stone::Black,
                Stone::Empty => Stone::Black,
            };
        }
    }

    // Last Move: Get last played position as JavaScript array
    pub fn get_last_move(&self) -> Option<JsValue> {
        if let Some((row, col)) = self.last_move {
            let result = vec![row as u32, col as u32];
            serde_wasm_bindgen::to_value(&result).ok()
        } else {
            None
        }
    }

    // Reset: Start new game
    pub fn reset(&mut self) {
        *self = GameState::new();
    }

    // Capture: Remove opponent group with no liberties, return count captured
    fn capture_group(&mut self, row: usize, col: usize) -> usize {
        let stone = self.board[row][col];
        if stone == Stone::Empty {
            return 0;
        }

        // BFS: Find all stones in the connected group
        let mut group = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(Position::new(row, col));
        group.insert((row, col));

        while let Some(pos) = queue.pop_front() {
            for neighbor in pos.neighbors() {
                if self.board[neighbor.row][neighbor.col] == stone
                    && !group.contains(&(neighbor.row, neighbor.col))
                {
                    group.insert((neighbor.row, neighbor.col));
                    queue.push_back(neighbor);
                }
            }
        }

        // Liberties: Check if group has any empty adjacent spaces
        let mut has_liberty = false;
        for &(r, c) in &group {
            if self.count_liberties(r, c) > 0 {
                has_liberty = true;
                break;
            }
        }

        // Capture: Remove group if no liberties
        if !has_liberty {
            for &(r, c) in &group {
                self.board[r][c] = Stone::Empty;
            }
            return group.len();
        }

        0
    }

    // Liberties: Count empty adjacent spaces for a stone/group
    fn count_liberties(&self, row: usize, col: usize) -> usize {
        let stone = self.board[row][col];
        if stone == Stone::Empty {
            return 0;
        }

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(Position::new(row, col));
        visited.insert((row, col));

        let mut liberties = 0;

        while let Some(pos) = queue.pop_front() {
            for neighbor in pos.neighbors() {
                if visited.contains(&(neighbor.row, neighbor.col)) {
                    continue;
                }

                match self.board[neighbor.row][neighbor.col] {
                    Stone::Empty => {
                        liberties += 1;
                        visited.insert((neighbor.row, neighbor.col));
                    }
                    s if s == stone => {
                        visited.insert((neighbor.row, neighbor.col));
                        queue.push_back(neighbor);
                    }
                    _ => {
                        visited.insert((neighbor.row, neighbor.col));
                    }
                }
            }
        }

        liberties
    }

    // Valid Move: Check if move is legal (not suicide, not ko, position empty)
    pub fn is_valid_move(&self, row: usize, col: usize) -> bool {
        if self.game_over {
            return false;
        }

        if row >= BOARD_SIZE || col >= BOARD_SIZE {
            return false;
        }

        if self.board[row][col] != Stone::Empty {
            return false;
        }

        // Test: Simulate move on temporary board
        let mut test_board = self.board.clone();
        test_board[row][col] = self.current_player;

        let opponent = match self.current_player {
            Stone::Black => Stone::White,
            Stone::White => Stone::Black,
            Stone::Empty => return false,
        };

        // Capture: Check if move would capture opponent stones
        let pos = Position::new(row, col);
        let mut would_capture = false;
        for neighbor in pos.neighbors() {
            if test_board[neighbor.row][neighbor.col] == opponent {
                // Check if neighbor group would be captured
                let mut group = HashSet::new();
                let mut queue = VecDeque::new();
                queue.push_back(neighbor);
                group.insert((neighbor.row, neighbor.col));

                while let Some(p) = queue.pop_front() {
                    for n in p.neighbors() {
                        if test_board[n.row][n.col] == opponent
                            && !group.contains(&(n.row, n.col))
                        {
                            group.insert((n.row, n.col));
                            queue.push_back(n);
                        }
                    }
                }

        // Liberties: Check if opponent group would have liberties after move
        let mut has_liberty = false;
        for &(r, c) in &group {
            let mut lib_visited = HashSet::new();
            let mut lib_queue = VecDeque::new();
            lib_queue.push_back(Position::new(r, c));
            lib_visited.insert((r, c));

            while let Some(p) = lib_queue.pop_front() {
                for n in p.neighbors() {
                    if lib_visited.contains(&(n.row, n.col)) {
                        continue;
                    }
                    match test_board[n.row][n.col] {
                        Stone::Empty => {
                            has_liberty = true;
                            break;
                        }
                        s if s == opponent => {
                            lib_visited.insert((n.row, n.col));
                            lib_queue.push_back(n);
                        }
                        _ => {
                            lib_visited.insert((n.row, n.col));
                        }
                    }
                }
                if has_liberty {
                    break;
                }
            }
            if has_liberty {
                break;
            }
        }

        if !has_liberty {
            would_capture = true;
            break;
        }
            }
        }

        // Liberties: Check if placed stone would have liberties
        let mut has_liberty = false;
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(pos);
        visited.insert((row, col));

        while let Some(p) = queue.pop_front() {
            for neighbor in p.neighbors() {
                if visited.contains(&(neighbor.row, neighbor.col)) {
                    continue;
                }
                match test_board[neighbor.row][neighbor.col] {
                    Stone::Empty => {
                        has_liberty = true;
                        break;
                    }
                    s if s == self.current_player => {
                        visited.insert((neighbor.row, neighbor.col));
                        queue.push_back(neighbor);
                    }
                    _ => {
                        visited.insert((neighbor.row, neighbor.col));
                    }
                }
            }
            if has_liberty {
                break;
            }
        }

        // Valid: Move is legal if it captures or has liberties
        would_capture || has_liberty
    }

    // Star Point: Check if position is a hoshi (star point)
    pub fn is_star_point(&self, row: usize, col: usize) -> bool {
        (row == 3 || row == 9 || row == 15) && (col == 3 || col == 9 || col == 15)
    }

    // Valid Moves: Get all legal moves as JavaScript array
    pub fn get_valid_moves(&self) -> JsValue {
        let mut valid_moves = Vec::new();
        if self.game_over {
            return serde_wasm_bindgen::to_value(&valid_moves).unwrap();
        }
        
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                if self.is_valid_move(row, col) {
                    valid_moves.push(vec![row as u32, col as u32]);
                }
            }
        }
        serde_wasm_bindgen::to_value(&valid_moves).unwrap()
    }

    // Label: Get column label (A-S)
    pub fn get_column_label(&self, col: usize) -> String {
        if col < 19 {
            char::from(65 + col as u8).to_string()
        } else {
            String::new()
        }
    }

    // Label: Get row label (19-1)
    pub fn get_row_label(&self, row: usize) -> String {
        (19 - row).to_string()
    }

    // Territory: Calculate territory for a player (empty spaces surrounded by that player's stones)
    fn calculate_territory(&self, player: Stone) -> usize {
        let mut visited = HashSet::new();
        let mut territory = 0;
        
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                if self.board[row][col] == Stone::Empty && !visited.contains(&(row, col)) {
                    let mut group = HashSet::new();
                    let mut queue = VecDeque::new();
                    queue.push_back(Position::new(row, col));
                    visited.insert((row, col));
                    group.insert((row, col));
                    
                    let mut has_black = false;
                    let mut has_white = false;
                    
                    // Flood fill to find all connected empty spaces
                    while let Some(p) = queue.pop_front() {
                        for neighbor in p.neighbors() {
                            match self.board[neighbor.row][neighbor.col] {
                                Stone::Empty => {
                                    if !visited.contains(&(neighbor.row, neighbor.col)) {
                                        visited.insert((neighbor.row, neighbor.col));
                                        group.insert((neighbor.row, neighbor.col));
                                        queue.push_back(neighbor);
                                    }
                                }
                                Stone::Black => {
                                    has_black = true;
                                }
                                Stone::White => {
                                    has_white = true;
                                }
                            }
                        }
                    }
                    
                    // Territory belongs to player if only their stones border it
                    if player == Stone::Black && has_black && !has_white {
                        territory += group.len();
                    } else if player == Stone::White && has_white && !has_black {
                        territory += group.len();
                    }
                }
            }
        }
        
        territory
    }
    
    // Score: Calculate final scores with komi (returns [black_score, white_score] as JsValue)
    pub fn calculate_scores(&self) -> JsValue {
        const KOMI: f64 = 6.5; // Standard komi for White
        
        if !self.game_over {
            return serde_wasm_bindgen::to_value(&vec![0.0, 0.0]).unwrap();
        }
        
        // Calculate territory
        let black_territory = self.calculate_territory(Stone::Black);
        let white_territory = self.calculate_territory(Stone::White);
        
        // Final scores: territory + captured stones + komi (for White)
        let black_score = black_territory as f64 + self.black_captured as f64;
        let white_score = white_territory as f64 + self.white_captured as f64 + KOMI;
        
        serde_wasm_bindgen::to_value(&vec![black_score, white_score]).unwrap()
    }
    
    // Board Data: Get all intersections with stone, star point, valid move, last move info
    pub fn get_board_data(&self) -> JsValue {
        #[derive(Serialize)]
        struct IntersectionData {
            row: usize,
            col: usize,
            stone: u8,
            is_star_point: bool,
            is_valid_move: bool,
            is_last_move: bool,
        }
        
        let mut board_data = Vec::new();
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                let stone_value = match self.board[row][col] {
                    Stone::Empty => 0,
                    Stone::Black => 1,
                    Stone::White => 2,
                };
                let is_star = self.is_star_point(row, col);
                let is_valid = !self.game_over && self.is_valid_move(row, col);
                let is_last_move = if let Some((r, c)) = self.last_move {
                    r == row && c == col && stone_value != 0
                } else {
                    false
                };
                
                board_data.push(IntersectionData {
                    row,
                    col,
                    stone: stone_value,
                    is_star_point: is_star,
                    is_valid_move: is_valid,
                    is_last_move,
                });
            }
        }
        serde_wasm_bindgen::to_value(&board_data).unwrap()
    }
}

// Init: Initialize WebAssembly module (set up panic hook for better error messages)
#[wasm_bindgen]
pub fn init() {
    console_error_panic_hook::set_once();
}