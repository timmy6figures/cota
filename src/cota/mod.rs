pub mod space;
pub use space::Space;

pub mod board;
pub use board::Board;

pub mod game;
pub use game::Game;

pub mod piece;
pub use piece::Piece;

pub mod file;
pub use file::File;

pub mod rank;
pub use rank::Rank;

pub mod position;
pub use position::Position;

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
            match self {
                Move::Piece(from, to) => {format!("{} => {}", from.to_string(), to.to_string())},
                Move::Lordswap(from, to) => {format!("{} <==> {}", from.to_string(), to.to_string())},
                Move::HorseSmall(from, intermediate, to) => {format!("{} => {} through {}", from.to_string(), to.to_string(), intermediate.to_string())},
                Move::HorseBig(from, intermediate_one, intermediate_two, to) => {format!("{} => {} through {} and {}", from.to_string(), to.to_string(), intermediate_one.to_string(), intermediate_two.to_string())},
                Move::Promote(_, _, _) => String::from("Promote"),
                Move::Resign => String::from("Resign"),
            }
    }
}

pub mod color;
pub use color::Color;
