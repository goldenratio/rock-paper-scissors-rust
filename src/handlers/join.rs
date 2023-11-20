use crate::AppState;
use actix_web::{post, web, HttpRequest, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct GameJoinRequestData {
    #[serde(rename = "gameId")]
    game_id: String,
    #[serde(rename = "playerName")]
    player_name: String,
}

#[derive(Serialize)]
enum GameJoinErrorType {
    InvalidGameId,
    PlayerAlreadyJoined,
    GenericError,
}

#[derive(Serialize)]
enum GameJoinResponseData {
    Success {
        #[serde(rename = "playerToken")]
        player_token: String,
    },
    Error {
        error: GameJoinErrorType,
    },
}

#[post("/join")]
async fn join(
    param_obj: web::Json<GameJoinRequestData>,
    _req: HttpRequest,
    state: web::Data<AppState>,
) -> impl Responder {
    let payload = param_obj.into_inner();
    println!("/join Received data: {:?}", payload);

    let mut game_creator = state.game_creator.lock().unwrap();
    let player_token = game_creator.create_new_player_token().unwrap();

    let response_data = GameJoinResponseData::Success {
        player_token,
    };
    return web::Json(response_data);
}
