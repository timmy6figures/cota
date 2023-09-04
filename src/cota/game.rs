use super::Board;

pub struct Game {
    player_one_name: String,
    player_two_name: String,

    board: Board,
    move_number: u8,
}

impl Game {

    pub fn new(name_one: String, name_two: String) -> Game {
        Game {
            player_one_name: name_one,
            player_two_name: name_two,
            board: Board::new(),
            move_number: 0,
        }
    }

}
