use std::sync::{Arc, Mutex};

use crate::create_game::GAME_LEN;
use crate::game_definitions::Definitions;

pub fn print(game: &Mutex<[[Definitions; GAME_LEN];GAME_LEN]>) {
    let game_guard = game.lock().unwrap();
    for (i,_) in game_guard.iter().enumerate() {
        for (j,_) in game_guard[i].iter().enumerate() {
            print!("|{}", game_guard[i][j]);
        }
        println!("");
    }
}
