use std::sync::{Mutex, Arc};   
use lazy_static::lazy_static;

use crate::game_definitions::Definitions;
use crate::create_game::GAME_LEN;
use crate::print::print;

lazy_static! {
    pub static ref CARDINAL_POINT: Mutex<String> = Mutex::new(String::from("south"));
}



pub fn move_up(game: &Mutex<[[Definitions;GAME_LEN];GAME_LEN]>) {
    let game_guard = game.lock().unwrap();
    for (i,_) in game_guard.iter().enumerate() {
        for (j,_) in game_guard.iter().enumerate() {
            if let Definitions::SnakeHead((i_,j_)) = game_guard[i][j]{
                match game_guard[i-1][j]{
                    Definitions::Snake((_,_)) => {
                        if (i-1,j) == (i_,j_) {
                            /* lose(); */
                        }
                    },
                    Definitions::Nothing => {
                        *CARDINAL_POINT.lock().unwrap() = "north".to_string();
                    },
                    Definitions::Apple => {
                        /* eat_apple(); */
                    },
                    _ => panic!(),
                    
                }
            }
        }
    }
}

pub fn move_down(game: &Mutex<[[Definitions;GAME_LEN];GAME_LEN]>) {
    let game_guard = game.lock().unwrap();
    for (i,_) in game_guard.iter().enumerate() {
        for (j,_) in game_guard.iter().enumerate() {
            if let Definitions::SnakeHead((i_,j_)) = game_guard[i][j]{
                match game_guard[i+1][j]{
                    Definitions::Snake((_,_)) => {
                        if (i+1,j) == (i_,j_) {
                            /* lose(); */
                        }
                    },
                    Definitions::Nothing => {
                        *CARDINAL_POINT.lock().unwrap() = "south".to_string();
                    },
                    Definitions::Apple => {
                        /* eat_apple(); */
                    },
                    _ => panic!(),
                    
                }
            }
        }
    }
}

pub fn move_left(game: &Mutex<[[Definitions;GAME_LEN];GAME_LEN]>) {
    let game_guard = game.lock().unwrap();
    for (i,_) in game_guard.iter().enumerate() {
        for (j,_) in game_guard.iter().enumerate() {
            if let Definitions::SnakeHead((i_,j_)) = game_guard[i][j]{
                match game_guard[i][j-1]{
                    Definitions::Snake((_,_)) => {
                        if (i,j-1) == (i_,j_) {
                            /* lose(); */
                        }
                    },
                    Definitions::Nothing => {
                        *CARDINAL_POINT.lock().unwrap() = "west".to_string();
                    },
                    Definitions::Apple => {
                        /* eat_apple(); */
                    },
                    _ => panic!(),
                    
                }
            }
        }
    }
}
pub fn move_right(game: &Mutex<[[Definitions;GAME_LEN];GAME_LEN]>) {
    let game_guard = game.lock().unwrap();
    for (i,_) in game_guard.iter().enumerate() {
        for (j,_) in game_guard.iter().enumerate() {
            if let Definitions::SnakeHead((i_,j_)) = game_guard[i][j]{
                match game_guard[i][j+1]{
                    Definitions::Snake((_,_)) => {
                        if (i,j+1) == (i_,j_) {
                            /* lose(); */
                        }
                    },
                    Definitions::Nothing => {
                        *CARDINAL_POINT.lock().unwrap() = "east".to_string();
                    },
                    Definitions::Apple => {
                        /* eat_apple(); */
                    },
                    _ => panic!(),
                    
                }
            }
        }
    }
}


 
pub fn snake_move(game: &Mutex<[[Definitions;GAME_LEN];GAME_LEN]>) {
    let card = CARDINAL_POINT.lock().unwrap();
    let mut game_guard = game.lock().unwrap();
    let mut tail: bool = false;
    let mut head: bool = false;

    for i in 0..GAME_LEN{
        for j in 0..GAME_LEN {
            if let  Definitions::Tail((i_,j_)) = game_guard[i][j] {
                if tail == false{
                    tail = true;
                    let (_i,_j) = match game_guard[i_][j_]{
                            Definitions::Snake((_i,_j)) => (_i,_j),
                            _ => panic!("else"),
                        };
                    game_guard[i_][j_] = Definitions::Tail((_i,_j));
                    game_guard[i][j] = Definitions::Nothing;
                }
            }
            if let Definitions::SnakeHead((i_,j_)) = game_guard[i][j]{
                if head == false{
                    head = true;
                    if *card == "south".to_string() {
                        game_guard[i+1][j] = Definitions::SnakeHead((i,j));
                        game_guard[i][j] = Definitions::Snake((i+1,j));
                        println!("i: {} j: {}",i,j);
                    }if *card == "north".to_string() {
                        game_guard[i-1][j] = Definitions::SnakeHead((i,j));
                        game_guard[i][j] = Definitions::Snake((i-1,j));
                    }if *card == "west".to_string() {
                        game_guard[i][j-1] = Definitions::SnakeHead((i,j));
                        game_guard[i][j] = Definitions::Snake((i,j-1));
                    }if *card == "east".to_string() {
                        game_guard[i][j+1] = Definitions::SnakeHead((i,j));
                        game_guard[i][j] = Definitions::Snake((i,j+1));
                    }
                }
            }
        }
    }
}

