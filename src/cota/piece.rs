use super::*;

#[derive(Copy, Clone)]
pub enum Piece {
    Lord(Color, Position, bool),
    Advisor(Color, Position),
    Marshal(Color, Position),
    Gun(Color, Position),
    Elephant(Color, Position),
    Horse(Color, Position),
    Kite(Color, Position),
    Troop(Color, Position),
    Chariot(Color, Position),
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
            Piece::Troop(color, pos) => {
                // Forward

                // If across the river, sideways as well 


            },
            Piece::Advisor(_, _) => panic!("TODO"),
            Piece::Marshal(_, _) => panic!("TODO"),
            Piece::Gun(_, _) => panic!("TODO"),
            Piece::Chariot(_, _) => panic!("TODO"),
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
            Piece::Troop(_, p) => {p},
            Piece::Advisor(_, p) => {p},
            Piece::Marshal(_, p) => {p},
            Piece::Gun(_, p) => {p},
            Piece::Chariot(_, p) => {p},
            Piece::Elephant(_, p) => {p},
            Piece::Horse(_, p) => {p},
            Piece::Kite(_, p) => {p},
        }
    }

    pub fn to_str(&self) -> &'static str {
        match *self {
            Self::Lord(_, _, _) => "Lord",
            Self::Advisor(_, _) => "Advisor",
            Self::Marshal(_, _) => "Marshal",
            Self::Gun(_, _) => "Gun",
            Self::Horse(_, _) => "Horse",
            Self::Elephant(_, _) => "Elephant", 
            Self::Kite(_, _)=> "Kite", 
            Self::Troop(_, _) => "Troop", 
            Self::Chariot(_, _) => "Chariot", 
        }
    }

    pub fn to_vietnamese_str(&self) -> &'static str {
        match *self {
            Self::Lord(_, _, _) => "CHÚA",
            Self::Advisor(_, _) => "SĨ",
            Self::Marshal(_, _) => "TƯỚNG",
            Self::Gun(_, _) => "PHÁO",
            Self::Horse(_, _) => "NGỰA",
            Self::Elephant(_, _) => "VOI", 
            Self::Kite(_, _)=> "DIỀU", 
            Self::Troop(_, _) => "LÍNH", 
            Self::Chariot(_, _) => "XE", 
        }
    }
    pub fn to_char(&self) -> char {
        match *self {
            Self::Lord(Color::White, _, _) => 'L',
            Self::Marshal(Color::White, _) => 'M',
            Self::Gun(Color::White, _) => 'G',
            Self::Horse(Color::White, _) => 'H',
            Self::Advisor(Color::White, _) => 'A',
            Self::Elephant(Color::White, _) => 'E', 
            Self::Kite(Color::White, _) => 'K', 
            Self::Troop(Color::White, _) => 'T', 
            Self::Chariot(Color::White, _) => 'C', 

            Self::Lord(Color::Black, _, _) => 'l',
            Self::Marshal(Color::Black, _) => 'm',
            Self::Gun(Color::Black, _) => 'g',
            Self::Horse(Color::Black, _) => 'h',
            Self::Advisor(Color::Black, _) => 'a',
            Self::Elephant(Color::Black, _) => 'e', 
            Self::Kite(Color::Black, _) => 'k', 
            Self::Troop(Color::Black, _) => 't', 
            Self::Chariot(Color::Black, _) => 'c', 
        }
    }

    pub fn to_fancy_char(&self) -> char {
        match *self {
            Self::Lord(Color::White, _, _) => '♔',
            Self::Gun(Color::White, _) => 'C',
            Self::Marshal(Color::White, _) => 'Q',
            Self::Horse(Color::White, _) => '♘',
            Self::Advisor(Color::White, _) => 'A',
            Self::Elephant(Color::White, _) => 'E', 
            Self::Kite(Color::White, _) => 'K', 
            Self::Troop(Color::White, _) => '♙', 
            Self::Chariot(Color::White, _) => 'C', 
Self::Lord(Color::Black, _, _) => '♚',
            Self::Marshal(Color::Black, _) => 'Q',
            Self::Gun(Color::Black, _) => 'c',
            Self::Horse(Color::Black, _) => '♞',
            Self::Advisor(Color::Black, _) => 'a',
            Self::Elephant(Color::Black, _) => 'e', 
            Self::Kite(Color::Black, _) => 'k', 
            Self::Troop(Color::Black, _) => '♟', 
            Self::Chariot(Color::Black, _) => 'c', 
        }
    }
}
