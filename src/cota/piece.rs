use super::*;

#[derive(Copy, Clone)]
pub enum Piece {
    Lord(Color, Position, bool),
    Assosciate(Color, Position),
    Queen(Color, Position),
    Cannon(Color, Position),
    Elephant(Color, Position),
    Horse(Color, Position),
    Kite(Color, Position),
    Pawn(Color, Position),
    Carriage(Color, Position),
}

impl Piece {
    
    /// Returns all of the potential moves of a piece, regardless of if they are legal or not.
    /// E.G. Ignoring all other pieces on the board
    pub fn get_potential_moves(&self, board: &Board) -> Vec<Move> {
        let mut v: Vec<Move> = Vec::new();

        let p: Position = self.get_position();
        println!("{}", p.to_string());
        match *self {
            Piece::Lord(_, _, just_kingswapped) => {
                let pos = p.surrounding();
                for i in pos {
                    v.push(Move::Piece(p, i));
                }
            },
            Piece::Pawn(_, _) => {
                // Forward
                

                // If across the river, sideways as well 


            },
            Piece::Assosciate(_, _) => panic!("TODO"),
            Piece::Queen(_, _) => panic!("TODO"),
            Piece::Cannon(_, _) => panic!("TODO"),
            Piece::Carriage(_, _) => panic!("TODO"),
            Piece::Elephant(_, _) => panic!("TODO"),
            Piece::Horse(_, _) => {
                if let Some(intermediate) = p.upper_left() {
                    if let Some(to) = intermediate.left() {
                        v.push(Move::HorseSmall(p, intermediate, to));
                        if let Some(after) = to.upper_left() { v.push(Move::HorseBig(p, intermediate, to, after)); }
                    }
                    if let Some(to) = intermediate.up() {
                        v.push(Move::HorseSmall(p, intermediate, to));
                        if let Some(after) = to.upper_left() { v.push(Move::HorseBig(p, intermediate, to, after)); }
                    }
                }
                if let Some(intermediate) = p.upper_right() {
                    if let Some(to) = intermediate.right() {
                        v.push(Move::HorseSmall(p, intermediate, to));
                        if let Some(after) = to.upper_right() { v.push(Move::HorseBig(p, intermediate, to, after)); }
                    }
                    if let Some(to) = intermediate.up() {
                        v.push(Move::HorseSmall(p, intermediate, to));
                        if let Some(after) = to.upper_right() { v.push(Move::HorseBig(p, intermediate, to, after)); }
                    }
                }
                if let Some(intermediate) = p.lower_left() {
                    if let Some(to) = intermediate.left() {
                        v.push(Move::HorseSmall(p, intermediate, to));
                        if let Some(after) = to.lower_left() { v.push(Move::HorseBig(p, intermediate, to, after)); }
                    }
                    if let Some(to) = intermediate.down() {
                        v.push(Move::HorseSmall(p, intermediate, to));
                        if let Some(after) = to.lower_left() { v.push(Move::HorseBig(p, intermediate, to, after)); }
                    }
                }
                if let Some(intermediate) = p.lower_right() {
                    if let Some(to) = intermediate.right() {
                        v.push(Move::HorseSmall(p, intermediate, to));
                        if let Some(after) = to.lower_right() { v.push(Move::HorseBig(p, intermediate, to, after)); }
                    }
                    if let Some(to) = intermediate.down() {
                        v.push(Move::HorseSmall(p, intermediate, to));
                        if let Some(after) = to.lower_right() { v.push(Move::HorseBig(p, intermediate, to, after)); }
                    }
                }

            },
            Piece::Kite(_, _) => panic!("TODO"),
            
        }
        v
    }

    /// Get all of the possible moves after energy considerations
    pub fn get_energy_moves(&self) {
        panic!("TODO");
    }

    pub fn get_legal_moves(&self) -> Vec<Move> {
        panic!("TODO");
        let v: Vec<Move> = Vec::new();
        v
    }

    fn is_legal_move(m: Move, board: Board) -> bool {
        panic!("TODO");
    }

    pub fn get_position(&self) -> Position {
        match *self {
            Piece::Lord(_, p, _) => {p},
            Piece::Pawn(_, p) => {p},
            Piece::Assosciate(_, p) => {p},
            Piece::Queen(_, p) => {p},
            Piece::Cannon(_, p) => {p},
            Piece::Carriage(_, p) => {p},
            Piece::Elephant(_, p) => {p},
            Piece::Horse(_, p) => {p},
            Piece::Kite(_, p) => {p},
        }
    }

    pub fn to_str(&self) -> &'static str {
        match *self {
            Self::Lord(_, _, _) => "Lord",
            Self::Assosciate(_, _) => "Associate",
            Self::Queen(_, _) => "Queen",
            Self::Cannon(_, _) => "Cannon",
            Self::Horse(_, _) => "Horse",
            Self::Elephant(_, _) => "Elephant", 
            Self::Kite(_, _)=> "Kite", 
            Self::Pawn(_, _) => "Pawn", 
            Self::Carriage(_, _) => "Carriage", 
        }
    }

    pub fn to_char(&self) -> char {
        match *self {
            Self::Lord(Color::White, _, _) => 'L',
            Self::Queen(Color::White, _) => 'Q',
            Self::Cannon(Color::White, _) => 'C',
            Self::Horse(Color::White, _) => 'H',
            Self::Assosciate(Color::White, _) => 'A',
            Self::Elephant(Color::White, _) => 'E', 
            Self::Kite(Color::White, _) => 'K', 
            Self::Pawn(Color::White, _) => 'P', 
            Self::Carriage(Color::White, _) => 'C', 

            Self::Lord(Color::Black, _, _) => 'l',
            Self::Queen(Color::Black, _) => 'q',
            Self::Cannon(Color::Black, _) => 'c',
            Self::Horse(Color::Black, _) => 'h',
            Self::Assosciate(Color::Black, _) => 'a',
            Self::Elephant(Color::Black, _) => 'e', 
            Self::Kite(Color::Black, _) => 'k', 
            Self::Pawn(Color::Black, _) => 'p', 
            Self::Carriage(Color::Black, _) => 'c', 
        }
    }

    pub fn to_fancy_char(&self) -> char {
        match *self {
            Self::Lord(Color::White, _, _) => '♔',
            Self::Cannon(Color::White, _) => 'C',
            Self::Queen(Color::White, _) => 'Q',
            Self::Horse(Color::White, _) => '♘',
            Self::Assosciate(Color::White, _) => 'A',
            Self::Elephant(Color::White, _) => 'E', 
            Self::Kite(Color::White, _) => 'K', 
            Self::Pawn(Color::White, _) => '♙', 
            Self::Carriage(Color::White, _) => 'C', 

            Self::Lord(Color::Black, _, _) => '♚',
            Self::Queen(Color::Black, _) => 'Q',
            Self::Cannon(Color::Black, _) => 'c',
            Self::Horse(Color::Black, _) => '♞',
            Self::Assosciate(Color::Black, _) => 'a',
            Self::Elephant(Color::Black, _) => 'e', 
            Self::Kite(Color::Black, _) => 'k', 
            Self::Pawn(Color::Black, _) => '♟', 
            Self::Carriage(Color::Black, _) => 'c', 
        }
    }
}
