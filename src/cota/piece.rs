use super::*;

#[derive(Copy, Clone)]
pub enum Piece {
    Emperor(Color),
    Assosciate(Color),
    Queen(Color),
    Cannon(Color),
    Elephant(Color),
    Horse(Color),
    Kite(Color),
    Pawn(Color),
    Carriage(Color),
}

impl Piece {
    
    pub fn to_str(&self) -> &'static str {
        match *self {
            Self::Emperor(_) => "Emperor",
            Self::Assosciate(_) => "King",
            Self::Queen(_) => "Queen",
            Self::Cannon(_) => "Cannon",
            Self::Horse(_) => "Horse",
            Self::Elephant(_) => "Elephant", 
            Self::Kite(_)=> "Kite", 
            Self::Pawn(_) => "Pawn", 
            Self::Carriage(_) => "Carriage", 
        }
    }

    pub fn to_char(&self) -> char {
        match *self {
            Self::Emperor(Color::White) => 'E',
            Self::Queen(Color::White) => 'Q',
            Self::Cannon(Color::White) => 'C',
            Self::Horse(Color::White) => 'H',
            Self::Assosciate(Color::White) => 'A',
            Self::Elephant(Color::White) => 'E', 
            Self::Kite(Color::White) => 'K', 
            Self::Pawn(Color::White) => 'P', 
            Self::Carriage(Color::White) => 'C', 

            Self::Emperor(Color::Black) => 'e',
            Self::Queen(Color::Black) => 'q',
            Self::Cannon(Color::Black) => 'c',
            Self::Horse(Color::Black) => 'h',
            Self::Assosciate(Color::Black) => 'a',
            Self::Elephant(Color::Black) => 'e', 
            Self::Kite(Color::Black) => 'k', 
            Self::Pawn(Color::Black) => 'p', 
            Self::Carriage(Color::Black) => 'c', 
        }
    }

    pub fn to_fancy_char(&self) -> char {
        match *self {
            Self::Emperor(Color::White) => '♔',
            Self::Cannon(Color::White) => 'C',
            Self::Queen(Color::White) => 'Q',
            Self::Horse(Color::White) => '♘',
            Self::Assosciate(Color::White) => 'A',
            Self::Elephant(Color::White) => 'E', 
            Self::Kite(Color::White) => 'K', 
            Self::Pawn(Color::White) => '♙', 
            Self::Carriage(Color::White) => 'C', 

            Self::Emperor(Color::Black) => '♚',
            Self::Queen(Color::Black) => 'Q',
            Self::Cannon(Color::Black) => 'c',
            Self::Horse(Color::Black) => '♞',
            Self::Assosciate(Color::Black) => 'a',
            Self::Elephant(Color::Black) => 'e', 
            Self::Kite(Color::Black) => 'k', 
            Self::Pawn(Color::Black) => '♟', 
            Self::Carriage(Color::Black) => 'c', 
        }
    }
}
