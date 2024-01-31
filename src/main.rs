mod admin_handlers;
mod app_state;
mod error_enums;
mod extractors;
mod game_creator;
mod game_entry;
mod gameplay_manager;
mod handlers;
mod player_action;
mod server_settings;
mod game_event_broadcast;

use std::sync::Arc;
use crate::admin_handlers::gameplay_info::gameplay_info;
use crate::handlers::create::create;
use crate::handlers::game_action::game_action;
use crate::handlers::game_events::game_events;
use crate::handlers::join::join;

use crate::admin_handlers::admin_status::admin_status;
use crate::app_state::AppState;
use crate::error_enums::{AppError, AppErrorData};
use crate::server_settings::ServerSettings;
use actix_cors::Cors;
use actix_web::error::{InternalError, JsonPayloadError};
use actix_web::middleware::Logger;
use actix_web::{get, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_static_files::ResourceFiles;
use crate::game_event_broadcast::Broadcaster;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[get("/")]
async fn index() -> impl Responder {
    return HttpResponse::Ok().body("Online");
}

fn generic_json_error(err: JsonPayloadError, req: &HttpRequest) -> Error {
    println!("generic JSON error {:?}, {:?}", err, req);
    let post_error = AppErrorData::Error {
        error_type: AppError::BadClientData,
    };
    return InternalError::from_response(err, HttpResponse::BadRequest().json(post_error)).into();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = ServerSettings::default();
    let admin_client_route = settings.admin_client_route.clone();

    let app_data = web::Data::new(AppState::new(&settings));
    let broadcast_data = Broadcaster::create();

    let socket_addr = settings.get_socket_addr();
    println!("server is running on, {:?}", socket_addr.to_string());
    println!(
        "admin UI: , {:?}",
        format!("http://{socket_addr}{admin_client_route}")
    );

    let server = HttpServer::new(move || {
        let static_admin_client_files = generate();
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method()
            .send_wildcard();

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(app_data.clone())
            .app_data(web::Data::from(Arc::clone(&broadcast_data)))
            .app_data(web::JsonConfig::default().error_handler(generic_json_error))
            .service(
                web::scope("/api")
                    .service(create)
                    .service(join)
                    .service(game_action)
                    .service(game_events),
            )
            .service(ResourceFiles::new(
                settings.admin_client_route.as_str(),
                static_admin_client_files,
            ))
            .service(
                web::scope("/admin-api")
                    .service(admin_status)
                    .service(gameplay_info),
            )
            .service(index)
    })
    .bind(socket_addr)?
    .run();

    // run server until stopped (either by ctrl-c or stop endpoint)
    server.await
}
