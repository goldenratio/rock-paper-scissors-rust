use crate::game_entry::GameEntry;
use crate::AppState;
use actix_web::{get, web, Responder};
use serde::Serialize;
use std::collections::HashMap;
use crate::extractors::admin_auth_token::AdminAuthentication;

#[derive(Serialize, Debug)]
struct GameplayInfoResponseData {
    #[serde(rename = "data")]
    data: HashMap<String, GameEntry>,
}

#[get("/gameplay_info")]
async fn gameplay_info(admin_auth_token: AdminAuthentication, state: web::Data<AppState>) -> impl Responder {
    println!("/gameplay_info {:?}", admin_auth_token);
    let gameplay_manager = state.gameplay_manager.lock().unwrap();
    let response_data = GameplayInfoResponseData {
        data: gameplay_manager.gameplay_data(),
    };
    return web::Json(response_data);
}
