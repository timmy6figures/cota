use super::file::*;
use super::rank::*;
use super::*;

#[derive(Clone, Copy)]
pub struct Space {
    marked: bool,
    kind: SpaceKind,
}

#[derive(Clone, Copy)]
pub enum SpaceKind {
    Occupied { piece: Piece, },
    Empty,
}

impl Space {

    pub fn new() -> Space {
        Space {
            marked: false,
            kind: SpaceKind::Empty,
        }
    }

    pub fn is_marked(&self) -> bool {
        self.marked
    }

    pub fn get_kind(&self) -> SpaceKind {
       self.kind 
    }

    pub fn get_piece(&self) -> Option<Piece> {
        match self.get_kind() {
            SpaceKind::Occupied { piece } => Some(piece),
            SpaceKind::Empty => None
        }
    }

    pub fn set_kind(&mut self, kind: SpaceKind) {
        self.kind = kind;
    }

    pub fn to_char(&self) -> char {
        match self.kind {
            SpaceKind::Occupied{piece} => piece.to_char(),
            SpaceKind::Empty{} => '-'
        }
    }

}
