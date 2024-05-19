use std::sync::{Mutex, Arc};
use crossterm::event::KeyCode;
use std::time::Duration;
use lazy_static::lazy_static;

use crate::create_game::GAME_LEN;

mod snake_movement;
mod create_game;
mod snake;
mod game_definitions;
mod print;

lazy_static! {
    pub static ref POSITION: Mutex<String> = Mutex::new(String::from("down"));
}

fn main() {


    let game  = create_game::create_game();
    let g1 = game.clone();
    let g2 = game.clone();
    let handle_1 = std::thread::spawn(move || {
        loop{
            while let Ok(event_result) = crossterm::event::poll(Duration::from_millis(100)){
                if event_result{
                    if let crossterm::event::Event::Key(event) = crossterm::event::read().unwrap() {
                    match event.code { 

                        KeyCode::Left => snake_movement::move_left(&g1),
                        KeyCode::Right => snake_movement::move_right(&g1),                                             
                        
                        _ => (),
                    }
                }
                }
            }
        }
    });

    let handle_2 = std::thread::spawn(move || {
        print::print(g2);
    });
    
    handle_1.join().unwrap();
    handle_2.join().unwrap();
}