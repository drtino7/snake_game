use std::sync::{Mutex, Arc};   
use lazy_static::lazy_static;

use crate::game_definitions::Definitions;
use crate::create_game::GAME_LEN;
use crate::additionals::look_concidences;

lazy_static! {
    pub static ref CARDINAL_POINT: Mutex<String> = Mutex::new(String::from("south"));
}


pub fn move_left(game: &Arc<Mutex<[[Definitions;GAME_LEN];GAME_LEN]>>) {
    let game_guard = game.lock().unwrap();

    for (i,_) in game_guard.iter().enumerate() {
        for (j,_) in game_guard.iter().enumerate() {
            if let  Definitions::SnakeHead = game_guard[i][j] {
                let mut card = CARDINAL_POINT.lock().unwrap();
                match card.as_str() {   
                    "south" => {
                        if !(game_guard[i][j+1] == Definitions::Snake){
                        *card =  "west".to_string();} },

                    "north" => {
                        if !(game_guard[i][j-1] == Definitions::Snake) {
                            *card =  "east".to_string();}},

                    "west" => {
                        if !(game_guard[i-1][j] == Definitions::Snake) {
                        *card =  "north".to_string();}},

                    "east" => {
                        if !(game_guard[i+1][j] == Definitions::Snake) {
                        *card =  "south".to_string();}},
                    _ => panic!(),
                }
            }
        }
    }
}



pub fn move_right(game: &Arc<Mutex<[[Definitions;GAME_LEN];GAME_LEN]>>) {
    let game_guard = game.lock().unwrap();

    for (i,_) in game_guard.iter().enumerate() {
        for (j,_) in game_guard.iter().enumerate() {
            if let  Definitions::SnakeHead = game_guard[i][j] {
                let mut card = CARDINAL_POINT.lock().unwrap();
                match card.as_str() {   
                    "south" => {
                        if !(game_guard[i][j-1] == Definitions::Snake){
                        *card =  "east".to_string();} },

                    "north" => {
                        if !(game_guard[i][j+1] == Definitions::Snake) {
                            *card =  "west".to_string();}},

                    "west" => {
                        if !(game_guard[i+1][j] == Definitions::Snake) {
                        *card =  "south".to_string();}},

                    "east" => {
                        if !(game_guard[i-1][j] == Definitions::Snake) {
                        *card =  "north".to_string();}},
                    _ => panic!(),
                }
            }
        }
    }
}

pub fn snake_move(game_clone: &Arc<Mutex<[[Definitions;GAME_LEN];GAME_LEN]>>, game: &Arc<Mutex<[[Definitions;GAME_LEN];GAME_LEN]>>) {
    let game_clone = game_clone.lock().unwrap();
    let mut game_guard = game.lock().unwrap();
    let card = CARDINAL_POINT.lock().unwrap();

    for (i,_) in game_clone.iter().enumerate() {
        for (j,_) in game_clone.iter().enumerate() {
            if let  Definitions::Tail((i_,j_)) = game_clone[i][j] {
                
            }
        }
    }
}

fn movement(pos: Vec<(usize,usize)>) {
    for action in pos{
        
    }
}

