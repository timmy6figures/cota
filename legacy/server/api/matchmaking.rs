use actix_rt;
use actix_web::HttpResponse;
use actix_web::{middleware, App, HttpServer};
use actix_web::{get, post};
use serde::{Deserialize, Serialize};

use crate::game_manager;

#[derive(Debug, Deserialize, Serialize)]
pub struct GeneralObj {
    data_type: String,
    message_data: MessageType,
}    

#[derive(Debug, Deserialize, Serialize)]
pub enum MessageType {
    GameInfo{
        host_id: String,
        password: String
    },
    JoinGameConfirm {
        game_id: String,
        password: String,
        guest_id: String
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GameInfo {
    data_type: String,
    data: String
}
impl GameInfo {
    pub fn new(data: String) -> GeneralObj {
        let data_type = String::from("gameInfo");
        GeneralObj {
            data_type,
            message_data: MessageType::GameInfo{host_id: String::from("f"), password: String::from("F")}
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JoinGameConfirm {
    data_type: String,
    data: String
}
impl JoinGameConfirm {
    pub fn new() -> GeneralObj {
        let data_type = String::from("joinGameConfirm");
        let data = String::from("success");
        GeneralObj {
            data_type,
            message_data: MessageType::JoinGameConfirm{game_id: String::from("f"), password: String::from("F"), guest_id: String::from("gID")}
        }
    }
}


#[get("/test")]
pub async fn test() -> HttpResponse {
    let g: GeneralObj = GameInfo::new(String::from("Game One"));
    HttpResponse::Ok()
        .content_type("application/json")
        .json(g)
}

// Returns a GameInfo object
#[post("/createGame")]
pub async fn create_game() -> HttpResponse {
    let g: GeneralObj = game_manager::new_game();
    HttpResponse::Ok()
        .content_type("application/json")
        .json(g)
}

#[post("/joinGame")]
pub async fn join_game() -> HttpResponse {
    let j: GeneralObj = game_manager::join_game();
    HttpResponse::Ok()
        .content_type("application/json")
        .json(j)
}
