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
enum GameActionErrorType {
    InvalidGameId,
    NotYourTurn,
    GenericError,
}

#[derive(Serialize)]
enum GameActionResponseData {
    Success {},
    Error { error: GameActionErrorType },
}

#[post("/game_action")]
async fn game_action(param_obj: web::Json<GameActionRequestData>) -> impl Responder {
    HttpResponse::Ok().body("hello game_action!")
}
