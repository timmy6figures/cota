use super::file::*;
use super::rank::*;
use super::*;

#[derive(Clone, Copy)]
pub enum Space {
    Occupied { piece: Piece},
    Empty,
}

impl Space {

    pub fn new() -> Space {
        Space::Empty
    }

    pub fn to_char(&self) -> char {
        match self {
            Space::Occupied{piece} => piece.to_fancy_char(),
            Space::Empty => '-'
        }
    }

}
