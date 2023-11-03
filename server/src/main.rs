#![allow(unused)]

mod cota;
mod game_manager;
use game_manager::*;

fn main() {
    
    let g = cota::new_game();
    g.valid_moves();

}
