use core::fmt;

use crate::snake_movement::CARDINAL_POINT;

#[derive(Clone,PartialEq, Eq, Copy)]
pub enum Definitions{
    Nothing,
    Snake,
    SnakeHead,
    Apple,
}

impl fmt::Display for Definitions{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let variant = match self {
            Definitions::Apple => "🍎",
            Definitions::Snake => "🐍",
            Definitions::Nothing => " ",

            Definitions::SnakeHead => {
               match CARDINAL_POINT.lock().unwrap().to_owned().as_str(){
                    "east" => "⬅️",
                    "west" => "➡️",
                    "north" => "⬆️",
                    "south" => "⬇️",
                    _ => panic!(),
               }
                
            },
            _ => panic!()
        };
        write!(f, "{}", variant)
    }
    
}
