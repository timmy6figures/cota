mod tests;
use tests


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


pub mod color;
pub use color::Color;



//fn main() {
//
//    let mut board = Board::new();
//    //board.set_to_starting_position();
//
//    let p = Piece::Horse(Color::White, position::A1);
//    board.force_place(p);
//
//    println!("Moves: ");
//    match board.get_piece_at(position::A1) {
//        None => {},
//        Some(p) => {
//            for mov in p.get_potential_moves(&board) {
//                println!("{}", mov.to_string());
//                
//                match mov {
//                    Move::Piece(f, t) => {
//                        board.mark(t);
//                    }
//                    _ => {}
//                }
//            }
//        },
//    }
//    println!("{}", board.to_string());
//
//    
//    
//    
//
//}


