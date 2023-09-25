use super::api::*;
use super::cota::Game;

pub fn run() {
    let games: Vec<Game> = Vec::new();

}

pub fn new_game() -> GeneralObj {
    GameInfo::new(String::from("Game One"))
}

pub fn join_game() -> GeneralObj {
    JoinGameConfirm::new()
}
