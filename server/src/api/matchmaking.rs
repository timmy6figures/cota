use actix_rt;
use actix_web::HttpResponse;
use actix_web::{middleware, App, HttpServer};
use actix_web::{get, post};
use serde::{Deserialize, Serialize};

use crate::gameManager;

#[derive(Debug, Deserialize, Serialize)]
pub struct GameInfo {
    data_type: String,
    data: String
}
impl GameInfo {
    pub fn new(data_type: String, data: String) -> GameInfo {
       GameInfo{
        data_type,
        data
       }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JoinGameConfirm {
    data_type: String,
    data: String
}
impl JoinGameConfirm {
    pub fn new() -> JoinGameConfirm {
        let data_type = String::from("joinGameConfirm");
        let data = String::from("success");
        JoinGameConfirm {
            data_type,
            data
        }
    }
}


#[get("/test")]
pub async fn test() -> HttpResponse {
    let g: GameInfo = GameInfo::new(String::from("type"), String::from("data"));
    HttpResponse::Ok()
        .content_type("application/json")
        .json(g)
}

// Returns a GameInfo object
#[post("/createGame")]
pub async fn create_game() -> HttpResponse {
    let g: GameInfo = GameInfo::new(String::from("type"), String::from("data"));
    let g: GameInfo = gameManager::new_game();
    HttpResponse::Ok()
        .content_type("application/json")
        .json(g)
}

#[post("/joinGame")]
pub async fn join_game() -> HttpResponse {
    let g: JoinGameConfirm = JoinGameConfirm::new();
    HttpResponse::Ok()
        .content_type("application/json")
        .json(g)
}
