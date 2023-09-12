use std::array;

use super::*;

pub struct Board {
    spaces: [[Space; 9]; 10],
}

impl Board {

    pub fn new() -> Board {
        Board {
            spaces: [[Space::new(); 9]; 10],
        }
    }

    pub fn starting_position() -> Board {
        let mut b: Board = Board::new();
        b.set_to_starting_position();
        b
    }

    pub fn set_to_starting_position(&mut self) {
        // Pawn
        self.force_place( Piece::Pawn(Color::White, position::A4));
        self.force_place( Piece::Pawn(Color::White, position::C4));
        self.force_place( Piece::Pawn(Color::White, position::D4));
        self.force_place( Piece::Pawn(Color::White, position::E4));
        self.force_place( Piece::Pawn(Color::White, position::F4));
        self.force_place( Piece::Pawn(Color::White, position::G4));
        self.force_place( Piece::Pawn(Color::White, position::I4));

        self.force_place( Piece::Pawn(Color::Black, position::A7));
        self.force_place( Piece::Pawn(Color::Black, position::C7));
        self.force_place( Piece::Pawn(Color::Black, position::D7));
        self.force_place( Piece::Pawn(Color::Black, position::E7));
        self.force_place( Piece::Pawn(Color::Black, position::F7));
        self.force_place( Piece::Pawn(Color::Black, position::G7));
        self.force_place( Piece::Pawn(Color::Black, position::I7));

        // Horse
        self.force_place( Piece::Horse(Color::White, position::B1));
        self.force_place( Piece::Horse(Color::White, position::H1));

        self.force_place( Piece::Horse(Color::Black, position::B9));
        self.force_place( Piece::Horse(Color::Black, position::H9));

        // Carrige 
        self.force_place( Piece::Carriage(Color::White, position::A1));
        self.force_place( Piece::Carriage(Color::White, position::I1));

        self.force_place( Piece::Carriage(Color::Black, position::A9));
        self.force_place( Piece::Carriage(Color::Black, position::I9));
        
        // Cannon 
        self.force_place( Piece::Cannon(Color::White, position::B3));
        self.force_place( Piece::Cannon(Color::White, position::H3));

        self.force_place( Piece::Cannon(Color::Black, position::B7));
        self.force_place( Piece::Cannon(Color::Black, position::H7));

        // Emperor
        self.force_place( Piece::Lord(Color::White, position::E1, false));
        self.force_place( Piece::Lord(Color::Black, position::E9, false));
        
        // Assosciate
        self.force_place( Piece::Assosciate(Color::White, position::D1));
        self.force_place( Piece::Assosciate(Color::Black, position::F9));
        
        // Assosciate
        self.force_place( Piece::Queen(Color::White, position::F1));
        self.force_place( Piece::Queen(Color::Black, position::D9));

        // Elephant
        self.force_place( Piece::Elephant(Color::White, position::G1));
        self.force_place( Piece::Elephant(Color::Black, position::C9));

        // Kite
        self.force_place( Piece::Kite(Color::White, position::C1));
        self.force_place( Piece::Kite(Color::Black, position::G9));


    }

    pub fn is_marked(&self, pos: Position) -> bool {
        let s: Option<Space> = self.spaces[pos.get_rank() as usize][pos.get_file() as usize].into();
        match s {
           None => {println!("ERR: tried to check out of bounds point");
           false}, 
           Some(sp) => {
               sp.is_marked()
           },
        }
    }

    pub fn get_piece_at(&self, pos: Position) -> Option<Piece> {
        let s: Option<Space> = self.spaces[pos.get_rank() as usize][pos.get_file() as usize].into();
        match s {
            None => {println!("ERR: tried to check out of bounds point"); panic!();}
            Some(sp) => {
                match sp.get_kind() {
                    space::SpaceKind::Empty => None,
                    space::SpaceKind::Occupied { piece } => Some(piece),
                            
                }
            } 
        }
    }
    
    /// Tries to place a piece at the given position
    pub fn place() {

    }

    /// Places a piece at the current position, regardless of what is there.
    pub fn force_place(&mut self, piece: Piece) {
        let pos = piece.get_position();
        let was_marked = self.is_marked(pos);
        let mut sp = Space::new();
        sp.set_kind(space::SpaceKind::Occupied{piece});
        self.spaces[pos.get_rank() as usize][pos.get_file() as usize] = sp;
    }

    pub fn mark(&self, pos: Position) {
        panic!("TODO");
        // TODO doesn't mark
        match self.spaces[pos.get_rank() as usize][pos.get_file() as usize].get_kind() {
            space::SpaceKind::Empty => {},
            space::SpaceKind::Occupied { piece }=> {},
        }
    }

    pub fn to_string(&self) -> String {
        let mut out_string = String::new();
        let mut reversed = self.spaces.clone();
        reversed.reverse();
        for file in reversed {
            for space in file {
                let t: Space = space.into();
                let c = match t.get_piece() {
                    None => '-',
                    Some(p) => p.to_fancy_char(),
                };
                out_string.push(c); 
                out_string.push(' '); 
            }
            out_string.push('\n');
        }
        out_string
    }

}
