use tokio_tungstenite::tungstenite::Result;
use tokio_tungstenite::WebSocketStream;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::Error as WsError;


pub enum Event {
    NewGame,
    PlacePiece,
}

pub async fn handle_event(r: &mut Result<WebSocketStream<TcpStream>, WsError>, s: &mut String) {


}
