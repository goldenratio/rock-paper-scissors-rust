mod app_state;
mod handlers;

use crate::app_state::{AppState, GameCreator};
use crate::handlers::create::create;
use crate::handlers::game_action::game_action;
use crate::handlers::game_status::game_status;
use crate::handlers::join::join;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::collections::HashMap;
use std::sync::Mutex;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("42")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(AppState {
        game_creator: Mutex::new(GameCreator { game_id: 0, player_token_id: 0 }),
        game_entries: Mutex::new(HashMap::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(create)
            .service(join)
            .service(game_action)
            .service(game_status)
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
