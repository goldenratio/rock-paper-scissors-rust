mod admin_handlers;
mod error_enums;
mod extractors;
mod game_creator;
mod game_entry;
mod gameplay_manager;
mod handlers;
mod player_action;

use crate::admin_handlers::gameplay_info::gameplay_info;
use crate::game_creator::GameCreator;
use crate::gameplay_manager::GameplayManager;
use crate::handlers::create::create;
use crate::handlers::game_action::game_action;
use crate::handlers::game_events::game_events;
use crate::handlers::join::join;

use crate::admin_handlers::admin_status::admin_status;
use crate::error_enums::{AppError, AppErrorData};
use actix_cors::Cors;
use actix_web::error::{InternalError, JsonPayloadError};
use actix_web::middleware::Logger;
use actix_web::{
    get, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder, ResponseError,
};
use actix_web_static_files::ResourceFiles;
use config::Config;
use std::collections::HashMap;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr};
use std::str::FromStr;
use std::sync::Mutex;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[derive(Debug)]
pub struct AdminInfo {
    pub admin_jwt_secret: String,
    pub admin_username: String,
    pub admin_password: String,
}

#[derive(Debug)]
pub struct AppState {
    pub game_creator: Mutex<GameCreator>,
    pub gameplay_manager: Mutex<GameplayManager>,
    pub admin_info: Mutex<AdminInfo>,
}

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
        admin_info: Mutex::new(AdminInfo {
            admin_jwt_secret: settings.get("admin_jwt_secret").unwrap().to_string(),
            admin_username: settings.get("admin_username").unwrap().to_string(),
            admin_password: settings.get("admin_password").unwrap().to_string(),
        }),
    });

    let server_port = settings.get("server_port").unwrap().parse::<u16>().unwrap();
    let use_ip_v6 = settings.get("use_server_ip_v6").unwrap().parse::<bool>().unwrap();
    let ip_addr = if use_ip_v6 {
        "::".to_string()
    } else {
        "127.0.0.1".to_string()
    };

    let socket_addr = SocketAddr::new(ip_addr.parse().unwrap(), server_port);
    println!("server is running on, {:?}", socket_addr.to_string());

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
            .app_data(web::JsonConfig::default().error_handler(generic_json_error))
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
