use super::*;

pub const A1: Position = Position::new(0, 0);
pub const A2: Position = Position::new(0, 1);
pub const A3: Position = Position::new(0, 2);
pub const A4: Position = Position::new(0, 3);
pub const A5: Position = Position::new(0, 4);
pub const A6: Position = Position::new(0, 5);
pub const A7: Position = Position::new(0, 6);
pub const A8: Position = Position::new(0, 7);
pub const A9: Position = Position::new(0, 8);
pub const A10: Position = Position::new(0, 9);

pub const B1: Position = Position::new(1, 0);
pub const B2: Position = Position::new(1, 1);
pub const B3: Position = Position::new(1, 2);
pub const B4: Position = Position::new(1, 3);
pub const B5: Position = Position::new(1, 4);
pub const B6: Position = Position::new(1, 5);
pub const B7: Position = Position::new(1, 6);
pub const B8: Position = Position::new(1, 7);
pub const B9: Position = Position::new(1, 8);
pub const B10: Position = Position::new(1, 9);

pub const C1: Position = Position::new(2, 0);
pub const C2: Position = Position::new(2, 1);
pub const C3: Position = Position::new(2, 2);
pub const C4: Position = Position::new(2, 3);
pub const C5: Position = Position::new(2, 4);
pub const C6: Position = Position::new(2, 5);
pub const C7: Position = Position::new(2, 6);
pub const C8: Position = Position::new(2, 7);
pub const C9: Position = Position::new(2, 8);
pub const C10: Position = Position::new(2, 9);

pub const D1: Position = Position::new(3, 0);
pub const D2: Position = Position::new(3, 1);
pub const D3: Position = Position::new(3, 2);
pub const D4: Position = Position::new(3, 3);
pub const D5: Position = Position::new(3, 4);
pub const D6: Position = Position::new(3, 5);
pub const D7: Position = Position::new(3, 6);
pub const D8: Position = Position::new(3, 7);
pub const D9: Position = Position::new(3, 8);
pub const D10: Position = Position::new(3, 9);

pub const E1: Position = Position::new(4, 0);
pub const E2: Position = Position::new(4, 1);
pub const E3: Position = Position::new(4, 2);
pub const E4: Position = Position::new(4, 3);
pub const E5: Position = Position::new(4, 4);
pub const E6: Position = Position::new(4, 5);
pub const E7: Position = Position::new(4, 6);
pub const E8: Position = Position::new(4, 7);
pub const E9: Position = Position::new(4, 8);
pub const E10: Position = Position::new(4, 9);

pub const F1: Position = Position::new(5, 0);
pub const F2: Position = Position::new(5, 1);
pub const F3: Position = Position::new(5, 2);
pub const F4: Position = Position::new(5, 3);
pub const F5: Position = Position::new(5, 4);
pub const F6: Position = Position::new(5, 5);
pub const F7: Position = Position::new(5, 6);
pub const F8: Position = Position::new(5, 7);
pub const F9: Position = Position::new(5, 8);
pub const F10: Position = Position::new(5, 9);

pub const G1: Position = Position::new(6, 0);
pub const G2: Position = Position::new(6, 1);
pub const G3: Position = Position::new(6, 2);
pub const G4: Position = Position::new(6, 3);
pub const G5: Position = Position::new(6, 4);
pub const G6: Position = Position::new(6, 5);
pub const G7: Position = Position::new(6, 6);
pub const G8: Position = Position::new(6, 7);
pub const G9: Position = Position::new(6, 8);
pub const G10: Position = Position::new(6, 9);

pub const H1: Position = Position::new(7, 0);
pub const H2: Position = Position::new(7, 1);
pub const H3: Position = Position::new(7, 2);
pub const H4: Position = Position::new(7, 3);
pub const H5: Position = Position::new(7, 4);
pub const H6: Position = Position::new(7, 5);
pub const H7: Position = Position::new(7, 6);
pub const H8: Position = Position::new(7, 7);
pub const H9: Position = Position::new(7, 8);
pub const H10: Position = Position::new(7, 9);

pub const I1: Position = Position::new(8, 0);
pub const I2: Position = Position::new(8, 1);
pub const I3: Position = Position::new(8, 2);
pub const I4: Position = Position::new(8, 3);
pub const I5: Position = Position::new(8, 4);
pub const I6: Position = Position::new(8, 5);
pub const I7: Position = Position::new(8, 6);
pub const I8: Position = Position::new(8, 7);
pub const I9: Position = Position::new(8, 8);
pub const I10: Position = Position::new(8, 9);


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Position {
    file: u8,
    rank: u8,
}

impl Position {

    pub const fn new(in_file: u8, in_rank: u8) -> Position {
        if in_file > 8 {
            panic!("Invalid Position creation")
        } else if in_rank > 9 {
            panic!("Invalid Position creation")
        }
        Position {
            file: in_file,
            rank: in_rank,
        }
    }

    pub fn get_file(&self) -> u8 {
        self.file
    }

    pub fn get_rank(&self) -> u8 {
        self.rank
    }

    pub fn up(&self) -> Option<Position> {
        if self.rank >= 9 {
            return None;
        } else {
            return Some(Position::new(self.file, self.rank + 1));
        }
    }

    pub fn down(&self) -> Option<Position> {
        if self.rank == 0 {
            return None;
        } else {
            return Some(Position::new(self.file, self.rank - 1));
        }
    }

    pub fn left(&self) -> Option<Position> {
        if self.file == 0 {
            return None;
        } else {
            return Some(Position::new(self.file - 1, self.rank));
        }
    }

    pub fn right(&self) -> Option<Position> {
        if self.file >= 8 {
            return None;
        } else {
            return Some(Position::new(self.file + 1, self.rank));
        }
    }

    pub fn upper_left(&self) -> Option<Position> {
        if let Some(i) = self.up() {
            return i.left()
        }
        None
    }

    pub fn upper_right(&self) -> Option<Position> {
        if let Some(i) = self.up() {
            return i.right()
        }
        None
    }

    pub fn lower_left(&self) -> Option<Position> {
        if let Some(i) = self.down() {
            return i.left()
        }
        None
    }

    pub fn lower_right(&self) -> Option<Position> {
        if let Some(i) = self.down() {
            return i.right()
        }
        None
    }
    
    pub fn orthogonals(&self) -> Vec<Position> {
        let mut vec = Vec::new();
        if let Some(i) = self.up() {
            vec.push(i);
        }
        if let Some(i) = self.down() {
            vec.push(i);
        }
        if let Some(i) = self.left() {
            vec.push(i);
        }
        if let Some(i) = self.right() {
            vec.push(i);
        }
        vec
    }

    pub fn diagonals(&self) -> Vec<Position> {
        let mut vec = Vec::new();
        if let Some(i) = self.upper_left() {
            vec.push(i);
        }
        if let Some(i) = self.upper_right() {
            vec.push(i);
        }
        if let Some(i) = self.lower_left() {
            vec.push(i);
        }
        if let Some(i) = self.lower_right() {
            vec.push(i);
        }
        vec
    }
    
    pub fn surrounding(&self) -> Vec<Position> {
        let mut d = self.diagonals();
        let mut o = self.orthogonals();
        d.append(&mut o);
        d
    }

    pub fn file_as_char(&self) -> char {
        match self.file {
            0 => 'A',
            1 => 'B',
            2 => 'C',
            3 => 'D',
            4 => 'E',
            5 => 'F',
            6 => 'G',
            7 => 'H',
            8 => 'I',
            _ => panic!(),
        }
    }

    pub fn rank_as_char(&self) -> char {
        match self.rank {
            0 => '1',
            1 => '2',
            2 => '3',
            3 => '4',
            4 => '5',
            5 => '6',
            6 => '7',
            7 => '8',
            8 => '9',
            9 => 'T',
            _ => panic!(),
        }
    }

    pub fn to_string(&self) -> String {
        let mut out = String::from(self.file_as_char());
        out.push(self.rank_as_char());
        return out;
    }

}
