use std::{env, io};

use actix_web::{middleware, App, HttpServer};

mod matchmaking;
pub use matchmaking::*;


#[actix_rt::main]
pub async fn run() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    //env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(matchmaking::create_game)
            .service(matchmaking::join_game)
            .service(matchmaking::test)
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await
}
