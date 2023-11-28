mod admin_handlers;
mod error_enums;
mod game_creator;
mod gameplay_manager;
mod handlers;

use crate::game_creator::GameCreator;
use crate::gameplay_manager::GameplayManager;
use crate::handlers::create::create;
use crate::handlers::game_action::game_action;
use crate::handlers::game_events::game_events;
use crate::handlers::join::join;
use crate::admin_handlers::shutdown::shutdown;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use actix_web_static_files::ResourceFiles;
use std::collections::HashMap;
use std::sync::Mutex;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[derive(Debug)]
pub struct AppState {
    pub game_creator: Mutex<GameCreator>,
    pub gameplay_manager: Mutex<GameplayManager>,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("42")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(AppState {
        game_creator: Mutex::new(GameCreator {
            game_id: 0,
            player_token_id: 0,
        }),
        gameplay_manager: Mutex::new(GameplayManager {
            game_entries: HashMap::new(),
        }),
    });

    let server = HttpServer::new(move || {
        let generated = generate();
        App::new()
            .app_data(app_data.clone())
            .service(
                web::scope("/api")
                    .service(create)
                    .service(join)
                    .service(game_action)
                    .service(game_events)
            )
            .service(ResourceFiles::new("/admin", generated))
            .service(
                web::scope("/admin-api")
                    .service(shutdown)
            )
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run();

    // run server until stopped (either by ctrl-c or stop endpoint)
    server.await
}
