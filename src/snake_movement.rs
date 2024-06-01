use std::sync::{Mutex,Arc};   
use lazy_static::lazy_static;
use rand::{self, Rng};

use crate::game_definitions::Definitions;
use crate::create_game::GAME_LEN;

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
                        if !((i-1,j) == (i_,j_)) {
                            lose();
                        }
                    },
                    Definitions::Nothing => {
                        *CARDINAL_POINT.lock().unwrap() = "north".to_string();
                    },
                    Definitions::Apple => {
                        *CARDINAL_POINT.lock().unwrap() = "north".to_string();
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
                        if !((i+1,j) == (i_,j_)) {
                            lose(); 
                        }
                    },
                    Definitions::Nothing => {
                        *CARDINAL_POINT.lock().unwrap() = "south".to_string();
                    },
                    Definitions::Apple => {
                        *CARDINAL_POINT.lock().unwrap() = "south".to_string();
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
                        if !((i,j-1) == (i_,j_)) {
                            lose(); 
                        }
                    },
                    Definitions::Nothing => {
                        *CARDINAL_POINT.lock().unwrap() = "west".to_string();
                    },
                    Definitions::Apple => {
                        *CARDINAL_POINT.lock().unwrap() = "west".to_string();
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
                        if !((i,j+1) == (i_,j_)) {
                            lose();
                        }
                    },
                    Definitions::Nothing => {
                        *CARDINAL_POINT.lock().unwrap() = "east".to_string();
                    },
                    Definitions::Apple => {
                        *CARDINAL_POINT.lock().unwrap() = "east".to_string();
                    },
                    _ => panic!(),
                    
                }
            }
        }
    }
}

pub fn gen_apple(game: &Mutex<[[Definitions;GAME_LEN];GAME_LEN]>) {
    let mut game_guard = game.lock().unwrap();
    fn gen() -> (usize,usize) {
        let i = rand::thread_rng().gen_range(0..GAME_LEN as u32);
        let j = rand::thread_rng().gen_range(0..GAME_LEN as u32);
        (i as usize,j as usize)
    }
    let mut i_j = gen();

    loop{
        match game_guard[i_j.0][i_j.1]{
            Definitions::Snake((_,_)) => {(i_j.0,i_j.1) = gen();},
            Definitions::Tail((_,_)) => {(i_j.0,i_j.1) = gen();},
            Definitions::SnakeHead((_,_)) => {(i_j.0,i_j.1) = gen();},
            _ => {
                game_guard[i_j.0][i_j.1] = Definitions::Apple;
                break;
            },
        }
    }
}
 
pub fn snake_move(game: &Arc<Mutex<[[Definitions;GAME_LEN];GAME_LEN]>>) {
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
            if let Definitions::SnakeHead((_,_)) = game_guard[i][j]{
                if head == false{
                    head = true;

                    if *card == "south".to_string() {
                        
                            if i == 15 {
                                lose();
                            }


                            if let Definitions::Apple = game_guard[i+1][j] {
                                game_guard[i+1][j] = Definitions::SnakeHead((i,j));
                                game_guard[i][j] = Definitions::Snake((i+1,j));
                                let mut flag = false;
                                
                                for i in 0..GAME_LEN {
                                    for j in 0..GAME_LEN{
                                        if flag == false{
                                        if let Definitions::Tail((_,_)) = game_guard[i][j] {
                                            if i > 0 && i < GAME_LEN{

                                                if let Definitions::Nothing = game_guard[i-1][j] {
                                                    game_guard[i-1][j] = Definitions::Tail((i,j));
                                                }
                                                else {   
                                                    if let Definitions::Nothing = game_guard[i+1][j] {
                                                        game_guard[i+1][j] = Definitions::Tail((i,j));
                                                    }
                                                    if let Definitions::Nothing = game_guard[i][j-1] {
                                                        game_guard[i][j-1] = Definitions::Tail((i,j));
                                                    }
                                                    if let Definitions::Nothing = game_guard[i][j+1] {
                                                        game_guard[i][j+1] = Definitions::Tail((i,j));
                                                    }
                                                }
                                                match game_guard[i][j]{
                                                    Definitions::Tail((i_,j_)) => {
                                                        game_guard[i][j] = Definitions::Snake((i_,j_));
                                                        flag = true;
                                                    } ,
                                                    _ => {},
                                                }
                                            }
                                        }
                                    
                                    }
                                }
                            }
                                fn gen() -> (usize,usize) {
                                let i = rand::thread_rng().gen_range(0..GAME_LEN as u32);
                                let j = rand::thread_rng().gen_range(0..GAME_LEN as u32);
                                (i as usize,j as usize)
                            }
                            let mut i_j = gen();

                            loop{
                                match game_guard[i_j.0][i_j.1]{
                                    Definitions::Snake((_,_)) => {(i_j.0,i_j.1) = gen();},
                                    Definitions::Tail((_,_)) => {(i_j.0,i_j.1) = gen();},
                                    Definitions::SnakeHead((_,_)) => {(i_j.0,i_j.1) = gen();},
                                    _ => {
                                        game_guard[i_j.0][i_j.1] = Definitions::Apple;
                                        break;
                                    },
                                }
                            }
                            }else{

                            game_guard[i+1][j] = Definitions::SnakeHead((i,j));
                            game_guard[i][j] = Definitions::Snake((i+1,j));
                            }

                    }
                        if *card == "north".to_string() {
                            if i == 0 {
                                lose();
                            }
                            if let Definitions::Apple = game_guard[i-1][j] {
                                game_guard[i-1][j] = Definitions::SnakeHead((i,j));
                                game_guard[i][j] = Definitions::Snake((i-1,j));
                                                    
                                let mut flag = false;
                                for i in 0..GAME_LEN {
                                    for j in 0..GAME_LEN{

                                        if flag == false{
                                        if let Definitions::Tail((_,_)) = game_guard[i][j] {
                                            if i > 0 && i < GAME_LEN-1{
                                                
                                                if let Definitions::Nothing = game_guard[i+1][j] {
                                                    game_guard[i+1][j] = Definitions::Tail((i,j));
                                                }
                                                else {   
                                                    if let Definitions::Nothing = game_guard[i-1][j] {
                                                        game_guard[i-1][j] = Definitions::Tail((i,j));
                                                    }
                                                    if let Definitions::Nothing = game_guard[i][j-1] {
                                                        game_guard[i][j-1] = Definitions::Tail((i,j));
                                                    }
                                                    if let Definitions::Nothing = game_guard[i][j+1] {
                                                        game_guard[i][j+1] = Definitions::Tail((i,j));
                                                    }
                                                }

                                                match game_guard[i][j]{
                                                    Definitions::Tail((i_,j_)) => {game_guard[i][j] = Definitions::Snake((i_,j_));
                                                    flag = true;
                                                    },
                                                    _ => {},
                                                }
                                            }
                                        }
                                    }
                                }
                                }
                                fn gen() -> (usize,usize) {
                                let i = rand::thread_rng().gen_range(0..GAME_LEN as u32);
                                let j = rand::thread_rng().gen_range(0..GAME_LEN as u32);
                                (i as usize,j as usize)
                            }
                            let mut i_j = gen();
                            loop{
                                match game_guard[i_j.0][i_j.1]{
                                    Definitions::Snake((_,_)) => {(i_j.0,i_j.1) = gen();},
                                    Definitions::Tail((_,_)) => {(i_j.0,i_j.1) = gen();},
                                    Definitions::SnakeHead((_,_)) => {(i_j.0,i_j.1) = gen();},
                                    _ => {
                                        game_guard[i_j.0][i_j.1] = Definitions::Apple;
                                        break;
                                    },
                                }
                            }
                            }else{

                            game_guard[i-1][j] = Definitions::SnakeHead((i,j));
                            game_guard[i][j] = Definitions::Snake((i-1,j));
                            }

                    }
                        if *card == "west".to_string() {
                            if j == 0 {
                                lose();
                            }
                            if let Definitions::Apple = game_guard[i][j-1] {
                                game_guard[i][j-1] = Definitions::SnakeHead((i,j));
                                game_guard[i][j] = Definitions::Snake((i,j-1));
                                let mut flag = false;
                                for i in 0..GAME_LEN {
                                    for j in 0..GAME_LEN{
                                        if flag == false{
                                        if let Definitions::Tail((_,_)) = game_guard[i][j] {
                                            if j > 0 && j < GAME_LEN - 1{

                                                if let Definitions::Nothing = game_guard[i][j+1] {
                                                    game_guard[i][j+1] = Definitions::Tail((i,j));
                                                }
                                                else {   
                                                    if let Definitions::Nothing = game_guard[i-1][j] {
                                                        game_guard[i-1][j] = Definitions::Tail((i,j));
                                                    }
                                                    if let Definitions::Nothing = game_guard[i][j-1] {
                                                        game_guard[i][j-1] = Definitions::Tail((i,j));
                                                    }
                                                    if let Definitions::Nothing = game_guard[i+1][j] {
                                                        game_guard[i+1][j] = Definitions::Tail((i,j));
                                                    }
                                                }
                                                match game_guard[i][j]{
                                                    Definitions::Tail((i_,j_)) => {game_guard[i][j] = Definitions::Snake((i_,j_));
                                                    flag = true;
                                                    },
                                                    _ => {},
                                                }
                                            }
                                        }
                                    
                                    }
                                }
                                }
                                fn gen() -> (usize,usize) {
                                let i = rand::thread_rng().gen_range(0..GAME_LEN as u32);
                                let j = rand::thread_rng().gen_range(0..GAME_LEN as u32);
                                (i as usize,j as usize)
                            }
                            let mut i_j = gen();

                            loop{
                                match game_guard[i_j.0][i_j.1]{
                                    Definitions::Snake((_,_)) => {(i_j.0,i_j.1) = gen();},
                                    Definitions::Tail((_,_)) => {(i_j.0,i_j.1) = gen();},
                                    Definitions::SnakeHead((_,_)) => {(i_j.0,i_j.1) = gen();},
                                    _ => {
                                        game_guard[i_j.0][i_j.1] = Definitions::Apple;
                                        break;
                                    },
                                }
                            }
                            }else{

                            game_guard[i][j-1] = Definitions::SnakeHead((i,j));
                            game_guard[i][j] = Definitions::Snake((i,j-1));
                            }

                    }
                        if *card == "east".to_string() {

                            if j == 15{
                                lose();
                            }

                            if let Definitions::Apple = game_guard[i][j+1] {
                                
                                game_guard[i][j+1] = Definitions::SnakeHead((i,j));
                                game_guard[i][j] = Definitions::Snake((i,j+1));
                                
                                let mut flag = false;
                                for i in 0..GAME_LEN {
                                    for j in 0..GAME_LEN{
                                        if flag == false {
                                        if let Definitions::Tail((_,_)) = game_guard[i][j] {
                                            if j > 0 && j < GAME_LEN - 1{
                                                if let Definitions::Nothing = game_guard[i][j-1] {
                                                    game_guard[i][j-1] = Definitions::Tail((i,j));
                                                }
                                                else {   
                                                    if let Definitions::Nothing = game_guard[i-1][j] {
                                                        game_guard[i-1][j] = Definitions::Tail((i,j));
                                                    }
                                                    if let Definitions::Nothing = game_guard[i][j+1] {
                                                        game_guard[i][j+1] = Definitions::Tail((i,j));
                                                    }
                                                    if let Definitions::Nothing = game_guard[i+1][j] {
                                                        game_guard[i+1][j] = Definitions::Tail((i,j));
                                                    }
                                                }
                                                match game_guard[i][j]{
                                                    Definitions::Tail((i_,j_)) => {game_guard[i][j] = Definitions::Snake((i_,j_));
                                                    flag = true;
                                                    },
                                                    _ => {},
                                                }
                                            }
                                        }
                                    
                                    }
                                }
                                }
                                fn gen() -> (usize,usize) {
                                let i = rand::thread_rng().gen_range(0..GAME_LEN as u32);
                                let j = rand::thread_rng().gen_range(0..GAME_LEN as u32);
                                (i as usize,j as usize)
                            }
                            let mut i_j = gen();

                            loop{
                                match game_guard[i_j.0][i_j.1]{
                                    Definitions::Snake((_,_)) => {(i_j.0,i_j.1) = gen();},
                                    Definitions::Tail((_,_)) => {(i_j.0,i_j.1) = gen();},
                                    Definitions::SnakeHead((_,_)) => {(i_j.0,i_j.1) = gen();},
                                    _ => {
                                        game_guard[i_j.0][i_j.1] = Definitions::Apple;
                                        break;
                                    },
                                }
                            }
                            }
                            else{
                                game_guard[i][j+1] = Definitions::SnakeHead((i,j));
                                game_guard[i][j] = Definitions::Snake((i,j+1));
                            }
                    }
               }
            }
        }
    }
}

fn lose(){
    println!("");
    println!("");
    println!("");
    println!("");
    println!("");
    println!("you lose");
    println!("");
    println!("");
    println!("");
    println!("");
    println!("");
    std::process::exit(0);
}
