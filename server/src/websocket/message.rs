use tokio_tungstenite::tungstenite::Message;

struct CotaMessage {
    message_type: String,
    data: String
}

pub fn handle_message(mess: &Message) -> Option<String> {
            match mess {
                Message::Text(t) => {
                    println!("Text");
                    return Some(String::from("Hello"));
                }
                _ => {}
            }
            Some(String::from("Hello"))
}
