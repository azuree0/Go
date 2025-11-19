mod board;
mod game;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use game::Game;

static mut CURRENT_GAME: Option<Game> = None;

#[wasm_bindgen]
pub fn init_game() {
    unsafe {
        CURRENT_GAME = Some(Game::new());
    }
}

#[wasm_bindgen]
pub fn make_move(x: i32, y: i32) -> bool {
    unsafe {
        if let Some(ref mut game) = CURRENT_GAME {
            game.make_move(x, y)
        } else {
            false
        }
    }
}

#[wasm_bindgen]
pub fn get_board() -> js_sys::Array {
    unsafe {
        let state = if let Some(ref game) = CURRENT_GAME {
            game.get_board_state()
        } else {
            vec![vec![0; board::BOARD_SIZE]; board::BOARD_SIZE]
        };
        
        let result = js_sys::Array::new();
        for row in state {
            let row_array = js_sys::Array::new();
            for cell in row {
                row_array.push(&cell.into());
            }
            result.push(&row_array);
        }
        result
    }
}

#[wasm_bindgen]
pub fn get_current_player() -> i32 {
    unsafe {
        if let Some(ref game) = CURRENT_GAME {
            game.get_current_player()
        } else {
            board::BLACK
        }
    }
}

#[wasm_bindgen]
pub fn is_game_over() -> bool {
    unsafe {
        if let Some(ref game) = CURRENT_GAME {
            game.is_game_over()
        } else {
            false
        }
    }
}

#[wasm_bindgen]
pub fn new_game() {
    unsafe {
        if let Some(ref mut game) = CURRENT_GAME {
            game.reset();
        } else {
            CURRENT_GAME = Some(Game::new());
        }
    }
}

#[wasm_bindgen]
pub fn pass_move() -> bool {
    unsafe {
        if let Some(ref mut game) = CURRENT_GAME {
            game.make_move(-1, -1)
        } else {
            false
        }
    }
}

#[wasm_bindgen]
pub fn undo_move() -> bool {
    unsafe {
        if let Some(ref mut game) = CURRENT_GAME {
            game.undo()
        } else {
            false
        }
    }
}

#[wasm_bindgen]
pub fn get_score() -> js_sys::Object {
    unsafe {
        let (black_score, white_score) = if let Some(ref game) = CURRENT_GAME {
            game.calculate_score()
        } else {
            (0, 0)
        };
        
        let result = js_sys::Object::new();
        js_sys::Reflect::set(&result, &"black".into(), &black_score.into()).unwrap();
        js_sys::Reflect::set(&result, &"white".into(), &white_score.into()).unwrap();
        result
    }
}

