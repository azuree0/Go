pub const BOARD_SIZE: usize = 19;
pub const EMPTY: i32 = 0;
pub const BLACK: i32 = 1;
pub const WHITE: i32 = 2;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }
}

pub struct Board {
    grid: [[i32; BOARD_SIZE]; BOARD_SIZE],
    ko: Option<Position>,
    last_move: Option<Position>,
}

impl Board {
    pub fn new() -> Self {
        Board {
            grid: [[EMPTY; BOARD_SIZE]; BOARD_SIZE],
            ko: None,
            last_move: None,
        }
    }

    pub fn get(&self, x: i32, y: i32) -> i32 {
        if x < 0 || x >= BOARD_SIZE as i32 || y < 0 || y >= BOARD_SIZE as i32 {
            return -1;
        }
        self.grid[y as usize][x as usize]
    }

    pub fn set(&mut self, x: i32, y: i32, color: i32) {
        if x >= 0 && x < BOARD_SIZE as i32 && y >= 0 && y < BOARD_SIZE as i32 {
            self.grid[y as usize][x as usize] = color;
        }
    }

    pub fn copy(&self) -> Board {
        Board {
            grid: self.grid,
            ko: self.ko,
            last_move: self.last_move,
        }
    }

    pub fn is_valid_move(&self, x: i32, y: i32, color: i32) -> bool {
        if x < 0 || x >= BOARD_SIZE as i32 || y < 0 || y >= BOARD_SIZE as i32 {
            return false;
        }
        if self.grid[y as usize][x as usize] != EMPTY {
            return false;
        }

        // Check ko rule
        if let Some(ko_pos) = self.ko {
            if ko_pos.x == x && ko_pos.y == y {
                return false;
            }
        }

        // Try the move
        let mut test_board = self.copy();
        test_board.set(x, y, color);

        // Check for suicide
        let captured = test_board.capture_groups(x, y, get_opponent(color));
        if captured.is_empty() {
            // Check if this move would capture any opponent stones
            let opponent_captured = test_board.capture_groups(x, y, color);
            if opponent_captured.is_empty() {
                // Check if the group has liberties
                if !test_board.has_liberties(x, y) {
                    return false;
                }
            }
        }

        true
    }

    pub fn make_move(&mut self, x: i32, y: i32, color: i32) -> bool {
        if !self.is_valid_move(x, y, color) {
            return false;
        }

        self.set(x, y, color);
        self.last_move = Some(Position::new(x, y));

        // Capture opponent stones
        let captured = self.capture_groups(x, y, get_opponent(color));
        let ko_pos = if captured.len() == 1 && captured[0].len() == 1 {
            Some(captured[0][0])
        } else {
            None
        };
        self.ko = ko_pos;

        true
    }

    fn capture_groups(&mut self, x: i32, y: i32, color: i32) -> Vec<Vec<Position>> {
        let mut captured = Vec::new();
        let mut visited = std::collections::HashSet::new();

        let neighbors = vec![
            Position::new(x - 1, y),
            Position::new(x + 1, y),
            Position::new(x, y - 1),
            Position::new(x, y + 1),
        ];

        for pos in neighbors {
            if visited.contains(&pos) {
                continue;
            }
            if self.get(pos.x, pos.y) == color {
                let group = self.get_group(pos.x, pos.y);
                if !self.group_has_liberties(&group) {
                    captured.push(group.clone());
                    for p in &group {
                        visited.insert(*p);
                        self.set(p.x, p.y, EMPTY);
                    }
                }
            }
        }

        captured
    }

    pub fn get_group(&self, x: i32, y: i32) -> Vec<Position> {
        let color = self.get(x, y);
        if color == EMPTY || color == -1 {
            return Vec::new();
        }

        let mut group = Vec::new();
        let mut visited = std::collections::HashSet::new();
        let mut stack = vec![Position::new(x, y)];

        while let Some(pos) = stack.pop() {
            if visited.contains(&pos) {
                continue;
            }
            visited.insert(pos);
            group.push(pos);

            let neighbors = vec![
                Position::new(pos.x - 1, pos.y),
                Position::new(pos.x + 1, pos.y),
                Position::new(pos.x, pos.y - 1),
                Position::new(pos.x, pos.y + 1),
            ];

            for n in neighbors {
                if !visited.contains(&n) && self.get(n.x, n.y) == color {
                    stack.push(n);
                }
            }
        }

        group
    }

    fn group_has_liberties(&self, group: &[Position]) -> bool {
        for pos in group {
            if self.has_liberties(pos.x, pos.y) {
                return true;
            }
        }
        false
    }

    fn has_liberties(&self, x: i32, y: i32) -> bool {
        let color = self.get(x, y);
        if color == EMPTY || color == -1 {
            return false;
        }

        let mut visited = std::collections::HashSet::new();
        let mut stack = vec![Position::new(x, y)];

        while let Some(pos) = stack.pop() {
            if visited.contains(&pos) {
                continue;
            }
            visited.insert(pos);

            let neighbors = vec![
                Position::new(pos.x - 1, pos.y),
                Position::new(pos.x + 1, pos.y),
                Position::new(pos.x, pos.y - 1),
                Position::new(pos.x, pos.y + 1),
            ];

            for n in neighbors {
                let neighbor_color = self.get(n.x, n.y);
                if neighbor_color == EMPTY {
                    return true;
                }
                if neighbor_color == color && !visited.contains(&n) {
                    stack.push(n);
                }
            }
        }

        false
    }

    pub fn get_valid_moves(&self, color: i32) -> Vec<Position> {
        let mut moves = Vec::new();
        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                if self.is_valid_move(x as i32, y as i32, color) {
                    moves.push(Position::new(x as i32, y as i32));
                }
            }
        }
        moves
    }

    pub fn get_territory_owner(&self, x: i32, y: i32) -> i32 {
        // Returns BLACK, WHITE, or EMPTY if neutral
        if self.get(x, y) != EMPTY {
            return EMPTY;
        }

        let mut visited = std::collections::HashSet::new();
        let mut stack = vec![Position::new(x, y)];
        let mut has_black = false;
        let mut has_white = false;

        while let Some(pos) = stack.pop() {
            if visited.contains(&pos) {
                continue;
            }
            visited.insert(pos);

            let neighbors = [
                Position::new(pos.x - 1, pos.y),
                Position::new(pos.x + 1, pos.y),
                Position::new(pos.x, pos.y - 1),
                Position::new(pos.x, pos.y + 1),
            ];

            for &n in &neighbors {
                if n.x < 0 || n.x >= BOARD_SIZE as i32 || n.y < 0 || n.y >= BOARD_SIZE as i32 {
                    continue;
                }
                let cell = self.get(n.x, n.y);
                match cell {
                    BLACK => has_black = true,
                    WHITE => has_white = true,
                    EMPTY => {
                        if !visited.contains(&n) {
                            stack.push(n);
                        }
                    }
                    _ => {}
                }
            }
        }

        // If territory is surrounded by only one color, that player controls it
        if has_black && !has_white {
            BLACK
        } else if has_white && !has_black {
            WHITE
        } else {
            EMPTY // Neutral territory
        }
    }
}

pub fn get_opponent(color: i32) -> i32 {
    if color == BLACK {
        WHITE
    } else {
        BLACK
    }
}

