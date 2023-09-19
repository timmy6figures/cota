#![allow(unused)]


mod handler;
use handler::*;
mod ws;
use ws::*;

type Result<T> = std::result::Result<T, Rejection>;
type Clients = Arc<Mutex<HashMap<String, Client>>>;

#[tokio::main]
async fn main() {

  let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

  let health_route = warp::path!("health").and_then(handler::health_handler);

  let publish = warp::path!("publish")
    .and(warp::body::json())
    .and(with_clients(clients.clone()))
    .and_then(handler::publish_handler);

  let ws_route = warp::path("ws")
    .and(warp::ws())
    .and(warp::path::param())
    .and(with_clients(clients.clone()))
    .and_then(handler::ws_handler);

  let routes = health_route
    .or(register_routes)
    .or(ws_route)
    .or(publish)
    .with(warp::cors().allow_any_origin());

  warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
  warp::any().map(move || clients.clone())
}






//fn main() {
//
//    let mut board = Board::new();
//    //board.set_to_starting_position();
//
//    let p = Piece::Horse(Color::White, position::A1);
//    board.force_place(p);
//
//    println!("Moves: ");
//    match board.get_piece_at(position::A1) {
//        None => {},
//        Some(p) => {
//            for mov in p.get_potential_moves(&board) {
//                println!("{}", mov.to_string());
//                
//                match mov {
//                    Move::Piece(f, t) => {
//                        board.mark(t);
//                    }
//                    _ => {}
//                }
//            }
//        },
//    }
//    println!("{}", board.to_string());
//
//    
//    
//    
//
//}


