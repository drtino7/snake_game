use std::sync::{Arc, Mutex};

use crate::game_definitions::Definitions;
use crate::create_game::GAME_LEN;

pub fn look_concidences(game: &Arc<Mutex<[[Definitions; GAME_LEN];GAME_LEN]>>, i: usize , j: usize) -> Vec<(usize,usize)> {
    let game_guard = game.lock().unwrap();
    let mut pos = vec![];
    // for (i,_) in game_guard.iter().enumerate() {
    //     for (j,_) in game_guard.iter().enumerate() {
    //         
    //     }
    // }

    if game_guard[i+1][j] == Definitions::Snake{
        pos.push((i+1,j));
    }
    if game_guard[i-1][j] == Definitions::Snake{
        pos.push((i-1,j));
    }
    if game_guard[i][j-1] == Definitions::Snake{
        pos.push((i,j-1));
    }
    if game_guard[i][j+1] == Definitions::Snake{
        pos.push((i,j+1));
    }
        pos
}
