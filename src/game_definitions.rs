use core::fmt;

use crate::snake_movement::CARDINAL_POINT;

#[derive(Clone,PartialEq, Eq, Copy)]
pub enum Definitions{
    Nothing,
    Snake((usize,usize)),
    SnakeHead((usize,usize)),
    Apple,
    Tail((usize,usize)),
}

impl fmt::Display for Definitions{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let variant = match self {
            Definitions::Apple => "🍎",
            Definitions::Snake((_,_)) => "🐍",
            Definitions::Nothing => "  ",
            Definitions::Tail((_,_)) => "🐍",
            Definitions::SnakeHead((_,_)) => { "h"
               // match CARDINAL_POINT.lock().unwrap().to_owned().as_str(){
               //      "east" => "⬅️ ",
               //      "west" => "➡️ ",
               //      "north" => "⬆️ ",
               //      "south" => "⬇️ ",
               //      _ => panic!(),
               // }
                
            },
            _ => panic!()
        };
        write!(f, "{}", variant)
    }
    
}
