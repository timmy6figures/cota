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

#[cfg(test)]
mod tests {
    
    
    #[test]
    fn rank_and_file_of_position() {
        let pos = crate::position::A1;
        assert_eq!(pos.get_file(), 0);
        assert_eq!(pos.get_rank(), 0);
        let pos = crate::position::C4;
        assert_eq!(pos.get_file(), 2);
        assert_eq!(pos.get_rank(), 3);
        let pos = crate::position::I10;
        assert_eq!(pos.get_file(), 8);
        assert_eq!(pos.get_rank(), 9);
    }

    #[test]
    fn rank_and_file_as_string() {
        let pos = crate::position::A1;
        assert_eq!(pos.to_string(), String::from("A1"));
        let pos = crate::position::C4;
        assert_eq!(pos.to_string(), String::from("C4"));
        let pos = crate::position::I10;
        assert_eq!(pos.to_string(), String::from("IT"));
    }

    #[test]
    fn position_to_direction() {
        let pos = crate::position::A1;
        assert_eq!(pos.up().unwrap(), crate::position::A2);
        let pos = crate::position::A1;
        assert_eq!(pos.right().unwrap(), crate::position::B1);
        let pos = crate::position::A1;
        assert_eq!(pos.upper_right().unwrap(), crate::position::B2);

    }

    #[test]
    fn position_out_of_bounds() {
        let pos1 = crate::position::A1;
        assert_ne!(pos1.up(), None);
        assert_eq!(pos1.down(), None);
        assert_eq!(pos1.left(), None);
        assert_ne!(pos1.right(), None);
        let pos2 = crate::position::A10;
        assert_eq!(pos2.up(), None);
        assert_ne!(pos2.down(), None);
        assert_eq!(pos2.left(), None);
        assert_ne!(pos2.right(), None);
        let pos3 = crate::position::I1;
        assert_ne!(pos3.up(), None);
        assert_eq!(pos3.down(), None);
        assert_ne!(pos3.left(), None);
        assert_eq!(pos3.right(), None);
        let pos4 = crate::position::I10;
        assert_eq!(pos4.up(), None);
        assert_ne!(pos4.down(), None);
        assert_ne!(pos4.left(), None);
        assert_eq!(pos4.right(), None);
    }
}

pub mod color;
pub use color::Color;
