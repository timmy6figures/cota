use std::array;

use super::*;

pub struct Board {
    spaces: [[Space; 9]; 9],
}

impl Board {

    pub fn new() -> Board {
        Board {
            spaces: [[Space::new(); 9]; 9],
        }
    }

    pub fn starting_position() -> Board {
        let mut b: Board = Board::new();
        b.to_starting_position();
        b
    }

    pub fn to_starting_position(&mut self) {
        // Pawn
        self.force_place( position::A4, Piece::Pawn(Color::White));
        self.force_place( position::C4, Piece::Pawn(Color::White));
        self.force_place( position::D4, Piece::Pawn(Color::White));
        self.force_place( position::E4, Piece::Pawn(Color::White));
        self.force_place( position::F4, Piece::Pawn(Color::White));
        self.force_place( position::G4, Piece::Pawn(Color::White));
        self.force_place( position::I4, Piece::Pawn(Color::White));

        self.force_place( position::A7, Piece::Pawn(Color::Black));
        self.force_place( position::C7, Piece::Pawn(Color::Black));
        self.force_place( position::D7, Piece::Pawn(Color::Black));
        self.force_place( position::E7, Piece::Pawn(Color::Black));
        self.force_place( position::F7, Piece::Pawn(Color::Black));
        self.force_place( position::G7, Piece::Pawn(Color::Black));
        self.force_place( position::I7, Piece::Pawn(Color::Black));

        // Horse
        self.force_place( position::B1, Piece::Horse(Color::White));
        self.force_place( position::H1, Piece::Horse(Color::White));

        self.force_place( position::B9, Piece::Horse(Color::Black));
        self.force_place( position::H9, Piece::Horse(Color::Black));

        // Carrige 
        self.force_place( position::A1, Piece::Carriage(Color::White));
        self.force_place( position::I1, Piece::Carriage(Color::White));

        self.force_place( position::A9, Piece::Carriage(Color::Black));
        self.force_place( position::I9, Piece::Carriage(Color::Black));
        
        // Cannon 
        self.force_place( position::B3, Piece::Cannon(Color::White));
        self.force_place( position::H3, Piece::Cannon(Color::White));

        self.force_place( position::B7, Piece::Cannon(Color::Black));
        self.force_place( position::H7, Piece::Cannon(Color::Black));

        // Emperor
        self.force_place( position::E1, Piece::Emperor(Color::White));
        self.force_place( position::E9, Piece::Emperor(Color::Black));
        
        // Assosciate
        self.force_place( position::D1, Piece::Assosciate(Color::White));
        self.force_place( position::F9, Piece::Assosciate(Color::Black));
        
        // Assosciate
        self.force_place( position::F1, Piece::Queen(Color::White));
        self.force_place( position::D9, Piece::Queen(Color::Black));

        // Elephant
        self.force_place( position::G1, Piece::Elephant(Color::White));
        self.force_place( position::C9, Piece::Elephant(Color::Black));

        // Kite
        self.force_place( position::C1, Piece::Kite(Color::White));
        self.force_place( position::G9, Piece::Kite(Color::Black));


    }

    pub fn piece_at(&self, pos: Position) -> Option<Piece> {
        let s: Space = self.spaces[pos.file() as usize][pos.rank() as usize].into();
        match s {
            Space::Empty => None,
            Space::Occupied { piece } => Some(piece),
        }
    }
    
    /// Tries to place a piece at the given position
    pub fn place() {

    }

    /// Places a piece at the current position, regardless of what is there.
    ///
    /// This is useful for an editor mode
    pub fn force_place(&mut self, pos: Position, piece: Piece) {
       self.spaces[pos.file() as usize][pos.rank() as usize] = Space::Occupied{ piece };
    }


    pub fn to_string(&self) -> String {
        let mut out_string = String::new();
        let mut reversed = self.spaces.clone();
        reversed.reverse();
        for file in reversed {
            for space in file {
                let t: Space = space.into();
                out_string.push(t.to_char()); 
                out_string.push(' '); 
            }
            out_string.push('\n');
        }
        out_string
    }

}
