mod app_state;
mod enum_types;
mod handlers;

use crate::app_state::{AppState, GameCreator, GameplayManager};
use crate::handlers::create::create;
use crate::handlers::game_action::game_action;
use crate::handlers::game_events::game_events;
use crate::handlers::join::join;
use actix_files as fs;
use actix_files::Files;
use actix_web::{get, web, App, HttpRequest, HttpServer, Responder};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;

#[get("/")]
async fn index(_req: HttpRequest) -> std::io::Result<fs::NamedFile> {
    let path: PathBuf = "./client/index.html".parse().unwrap();
    Ok(fs::NamedFile::open(path)?)
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

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(
                web::scope("/api")
                    .service(create)
                    .service(join)
                    .service(game_action)
                    .service(game_events),
            )
            .service(Files::new("/", "./client/").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
