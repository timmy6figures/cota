use super::Board;

pub struct Game {
    player_one_name: String,
    player_two_name: String,

    board: Board,
    move_number: u8,
}

impl Game {

    pub fn new(player_name_one: String, player_name_two: String) -> Game {
        Game {
            player_one_name: player_name_one,
            player_two_name: player_name_two,
            board: Board::new(),
            move_number: 0,
        }
    }

    pub fn current_move(&self) -> u8{
       self.move_number
    }

    pub fn fen(&self) {
        panic!("TODO");
    }
    pub fn move_history(&self) {
        panic!("TODO");
    }

    pub fn play_move(&self) {
        panic!("TODO");
    }
}
