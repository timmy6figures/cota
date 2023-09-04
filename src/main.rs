#![allow(unused)]
use yew::prelude::*;

mod cota;
use cota::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <img src="/static/background.jpg" width="900px" />
            <img src="/static/pieces/horse.png" width="900px" />
            <img src="/static/pieces/crown.png" width="900px" />

        </>
    }
}

fn main() {

    let player_name = String::from("Jarvis");
    let com_name = String::from("COM");

//    let g = Game::new(player_name, com_name);
//
//    let pos = Position::new(8,8);
//    let mut low_pos = match pos.above() {
//        Some(i) => i,
//        None => Position::new(1,1),
//    };
//
//
//    println!("{}", pos.to_string());
//    for pos in pos.orthogonals().iter() {
//        println!("{}", pos.to_string());
//    }
//    println!("{}", low_pos.to_string());

    let mut b = Board::new();
    println!("{}", b.to_string());
    b.to_starting_position();
    println!("After place");
    println!("{}", b.to_string());

    println!("{}", b.piece_at(position::A1).unwrap().to_char());

//    yew::Renderer::<App>::new().render();
}
