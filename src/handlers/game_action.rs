use crate::error_enums::GameActionError;
use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct GameActionRequestData {
    #[serde(rename = "actionType")]
    action_type: String,
    #[serde(rename = "playerToken")]
    player_token: String,
}

#[derive(Serialize)]
enum GameActionResponseData {
    #[serde(rename = "data")]
    Success {},
    #[serde(rename = "data")]
    Error {
        #[serde(rename = "error")]
        error_type: GameActionError,
    },
}

#[post("/game_action")]
async fn game_action(param_obj: web::Json<GameActionRequestData>) -> impl Responder {
    HttpResponse::Ok().body("hello game_action!")
}
