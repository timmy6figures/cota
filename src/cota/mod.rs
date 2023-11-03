pub mod space;
pub use space::Space;

pub mod board;
pub use board::Board;

pub mod color;
pub use color::Color;

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
pub mod tests;


pub fn new_game() -> Game {
    Game::new(String::from("Player One"), String::from("Player Two"))
}









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


