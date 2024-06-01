use crossterm::event::KeyCode;
use std::time::Duration;

mod snake_movement;
mod create_game;
mod game_definitions;
mod print;

fn main() {


    let game  = create_game::create_game();
    let g1 = game.clone();
    let g2 = game.clone();
    let g3 = game.clone();
    let handle_1 = std::thread::spawn(move || {
        loop{
            while let Ok(event_result) = crossterm::event::poll(Duration::from_millis(100)){
                if event_result{
                    if let crossterm::event::Event::Key(event) = crossterm::event::read().unwrap() {
                    match event.code { 

                        KeyCode::Left => snake_movement::move_left(&g1),
                        KeyCode::Right => snake_movement::move_right(&g1),                                             
                        KeyCode::Up => snake_movement::move_up(&g1),
                        KeyCode::Down => snake_movement::move_down(&g1),
                        _ => (),
                    }
                }
                }
            }
        }
    });

    let handle_2 = std::thread::spawn(move || {
        loop{
            print::print(&g2);
            std::thread::sleep(std::time::Duration::from_millis(1500));
        }
    });

    let handle_3 = std::thread::spawn(move || {
        loop{
            snake_movement::snake_move(&g3);
            std::thread::sleep(std::time::Duration::from_millis(1500));
        }
    });
    
    handle_1.join().unwrap();
    handle_2.join().unwrap();
    handle_3.join().unwrap();
}
