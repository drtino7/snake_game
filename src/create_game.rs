use std::sync::{Mutex,Arc};
use crate::game_definitions::Definitions;

pub const GAME_LEN: usize = 16;
pub fn create_game() -> Arc<Mutex<[[Definitions;GAME_LEN];GAME_LEN]>> {
let game: Arc<Mutex<[[Definitions;GAME_LEN];GAME_LEN]>> = Arc::new(Mutex::new([[Definitions::Nothing;GAME_LEN];GAME_LEN]));
    game.lock().unwrap()[8][8] = Definitions::SnakeHead;
    game.lock().unwrap()[7][8] = Definitions::Snake;
    game
    
}
