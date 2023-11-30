use std::collections::HashMap;
use crate::AppState;
use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;
use crate::error_enums::GameplayInfoError;
use crate::game_entry::GameEntry;

#[derive(Serialize, Debug)]
enum GameplayInfoResponseData {
    #[serde(rename = "data")]
    Success {
        #[serde(rename = "gameplayData")]
        gameplay_data: HashMap<String, GameEntry>,
    },
    #[serde(rename = "data")]
    Error {
        #[serde(rename = "error")]
        error_type: GameplayInfoError,
    },
}

#[get("/gameplay_info")]
async fn gameplay_info(state: web::Data<AppState>) -> impl Responder {
    println!("/gameplay_info");
    let gameplay_manager = state.gameplay_manager.lock().unwrap();
    let response_data = GameplayInfoResponseData::Success {
        gameplay_data: gameplay_manager.gameplay_data()
    };
    return web::Json(response_data);
}
