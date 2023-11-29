use crate::error_enums::CreateGameError;
use crate::AppState;
use actix_web::{get, web, Responder};
use serde::Serialize;

#[derive(Serialize, Debug)]
enum CreateGameResponseData {
    #[serde(rename = "data")]
    Success {
        #[serde(rename = "gameId")]
        game_id: String,
    },
    #[serde(rename = "data")]
    Error {
        #[serde(rename = "error")]
        error_type: CreateGameError,
    },
}

#[get("/create")]
async fn create(state: web::Data<AppState>) -> impl Responder {
    let mut game_creator = state.game_creator.lock().unwrap();
    let mut gameplay_manager = state.gameplay_manager.lock().unwrap();

    let response_data = match game_creator.create_new_game() {
        Ok(game_id) => match gameplay_manager.register_game(&game_id) {
            Ok(_) => CreateGameResponseData::Success { game_id },
            Err(error_type) => CreateGameResponseData::Error { error_type },
        },
        Err(_) => CreateGameResponseData::Error {
            error_type: CreateGameError::GenericError,
        },
    };

    println!("/create {:?}", response_data);
    return web::Json(response_data);
}
