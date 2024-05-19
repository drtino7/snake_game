use std::sync::{Mutex, Arc};   
use lazy_static::lazy_static;

use crate::game_definitions::Definitions;
use crate::create_game::GAME_LEN;


lazy_static! {
    pub static ref CARDINAL_POINT: Mutex<String> = Mutex::new(String::from("south"));
}


pub fn move_left(game: &Arc<Mutex<[[Definitions;GAME_LEN];GAME_LEN]>>) {
    for (i,_) in game.lock().unwrap().iter().enumerate() {
        for (j,_) in game.lock().unwrap().iter().enumerate() {
            if let  Definitions::SnakeHead = game.lock().unwrap()[i][j] {
                if !(game.lock().unwrap()[i][j-1] == Definitions::Snake) {
                    match CARDINAL_POINT.lock().unwrap().to_owned().as_str() {   
                        "south" => {CARDINAL_POINT.lock().unwrap().replace("south", "west");},
                        "north" => {CARDINAL_POINT.lock().unwrap().replace("north", "east");},
                        "west" => {CARDINAL_POINT.lock().unwrap().replace("west", "north");},
                        "east" => {CARDINAL_POINT.lock().unwrap().replace("east", "south");},
                        _ => panic!(),
                    }
                }                    
            }
        }
    }
}


pub fn move_right(game: &Arc<Mutex<[[Definitions;GAME_LEN];GAME_LEN]>>) {
    for (i,_) in game.lock().unwrap().iter().enumerate() {
        for (j,_) in game.lock().unwrap().iter().enumerate() {
            if let  Definitions::SnakeHead = game.lock().unwrap()[i][j] {
                if !(game.lock().unwrap()[i][j+1] == Definitions::Snake) {
                    match CARDINAL_POINT.lock().unwrap().to_owned().as_str() {   
                        "south" => {CARDINAL_POINT.lock().unwrap().replace("south", "east");},
                        "north" => {CARDINAL_POINT.lock().unwrap().replace("north", "west");},
                        "west" => {CARDINAL_POINT.lock().unwrap().replace("west", "south");},
                        "east" => {CARDINAL_POINT.lock().unwrap().replace("east", "north");},
                        _ => panic!(),
                    }
                }                    
            }
        }
    }
}





