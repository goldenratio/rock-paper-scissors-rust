mod admin_handlers;
mod error_enums;
mod game_creator;
mod gameplay_manager;
mod handlers;
mod game_entry;
mod player_action;

use crate::admin_handlers::shutdown::shutdown;
use crate::game_creator::GameCreator;
use crate::gameplay_manager::GameplayManager;
use crate::handlers::create::create;
use crate::handlers::game_action::game_action;
use crate::handlers::game_events::game_events;
use crate::handlers::join::join;

use crate::error_enums::{AppError, AppErrorData};
use actix_web::error::{InternalError, JsonPayloadError};
use actix_web::{
    get, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder, ResponseError,
};
use actix_web_static_files::ResourceFiles;
use config::Config;
use std::collections::HashMap;
use std::sync::Mutex;
use actix_web::middleware::Logger;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[derive(Debug)]
pub struct AppState {
    pub game_creator: Mutex<GameCreator>,
    pub gameplay_manager: Mutex<GameplayManager>,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Online")
}

fn generic_app_error(err: JsonPayloadError, _req: &HttpRequest) -> Error {
    println!("{:?}", err);
    let post_error = AppErrorData::Error {
        error_type: AppError::BadClientData,
    };
    InternalError::from_response(err, HttpResponse::BadRequest().json(post_error)).into()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = Config::builder()
        .add_source(config::File::with_name("src/config"))
        .build()
        .unwrap()
        .try_deserialize::<HashMap<String, String>>()
        .unwrap();

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
        let static_admin_client_files = generate();
        App::new()
            .wrap(Logger::default())
            .app_data(app_data.clone())
            .app_data(web::JsonConfig::default().error_handler(generic_app_error))
            .service(
                web::scope("/api")
                    .service(create)
                    .service(join)
                    .service(game_action)
                    .service(game_events),
            )
            .service(ResourceFiles::new(
                settings.get("admin_client_route").unwrap(),
                static_admin_client_files,
            ))
            .service(web::scope("/admin-api").service(shutdown))
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run();

    // run server until stopped (either by ctrl-c or stop endpoint)
    server.await
}
