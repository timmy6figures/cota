#![allow(unused)]

mod cota;

fn main() {
    
    let game = cota::new_game();

    // Stats about the game
    game.current_move();
    //game.fen();
    //game.move_history();

    // Interactions with the game
    //game.valid_moves();
    game.play_move();
}
