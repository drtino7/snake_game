use core::fmt;

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
            Definitions::Tail((_,_)) => "TA",
            Definitions::SnakeHead((_,_)) => "🙂",
        };
        write!(f, "{}", variant)
    }
    
}
