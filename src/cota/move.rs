
pub enum Move {
    Piece(Position, Position),
    Lordswap(Position, Position),
    HorseSmall(Position, Position, Position), // From, Intermediate, To
    HorseBig(Position, Position, Position, Position), // From, Intermediate one, Intermediate two, To 
    Promote(Piece, Position, Position), 
    Resign
}

impl Move {

    pub fn to_string(&self) -> String {
            use Move::*;
            match self {
                Piece(from, to) => {format!("{} => {}", from.to_string(), to.to_string())},
                Lordswap(from, to) => {format!("{} <==> {}", from.to_string(), to.to_string())},
                HorseSmall(from, intermediate, to) => {format!("{} => {} through {}", from.to_string(), to.to_string(), intermediate.to_string())},
                HorseBig(from, intermediate_one, intermediate_two, to) => {format!("{} => {} through {} and {}", from.to_string(), to.to_string(), intermediate_one.to_string(), intermediate_two.to_string())},
                Promote(_, _, _) => String::from("Promote"),
                Resign => String::from("Resign"),
            }
    }
}
