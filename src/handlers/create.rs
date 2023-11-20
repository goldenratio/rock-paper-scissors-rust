use crate::app_state::AppState;
use actix_web::{get, web, Responder};
use serde::Serialize;

#[derive(Serialize, Debug)]
enum CreateGameErrorType {
    GenericError,
}

#[derive(Serialize, Debug)]
enum CreateGameResponseData {
    Success {
        #[serde(rename = "gameId")]
        game_id: String,
    },
    Error {
        error: CreateGameErrorType,
    },
}

#[get("/create")]
async fn create(state: web::Data<AppState>) -> impl Responder {
    let mut game_creator = state.game_creator.lock().unwrap();

    let new_game_result = game_creator.create_new_game();

    let response_data = match new_game_result {
        Ok(game_id) => {
            // println!("do something with game id {}", game_id);
            // Do something with game_id
            CreateGameResponseData::Success { game_id }
        },
        Err(_) => CreateGameResponseData::Error {
            error: CreateGameErrorType::GenericError,
        },
    };

    println!("/create {:?}", response_data);
    return web::Json(response_data);
}
