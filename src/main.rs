#![allow(unused)]

mod cota;
use cota::*;

mod api;
use crate::api::*;

fn main() {

    let mut board = Board::new();
    //board.set_to_starting_position();

    let p = Piece::Horse(Color::White, position::A1);
    board.force_place(p);

    println!("Moves: ");
    match board.get_piece_at(position::A1) {
        None => {},
        Some(p) => {
            for mov in p.get_potential_moves(&board) {
                println!("{}", mov.to_string());
                
                match mov {
                    Move::Piece(f, t) => {
                        board.mark(t);
                    }
                    _ => {}
                }
            }
        },
    }
    println!("{}", board.to_string());

    
    
    

}


