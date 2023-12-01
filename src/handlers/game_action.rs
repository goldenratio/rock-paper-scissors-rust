use crate::error_enums::GameActionError;
use crate::player_action::PlayerAction;
use crate::AppState;
use actix_web::{post, web, Responder};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Deserialize, Debug)]
struct GameActionRequestData {
    #[serde(rename = "actionType")]
    action_type: String,
    #[serde(rename = "playerToken")]
    player_token: String,
    #[serde(rename = "gameId")]
    game_id: String,
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
async fn game_action(
    param_obj: web::Json<GameActionRequestData>,
    state: web::Data<AppState>,
) -> impl Responder {
    let payload = param_obj.into_inner();
    println!("/game_action Received data: {:?}", payload);

    let player_action_param = PlayerAction::from_str(payload.action_type.as_str());
    if player_action_param.is_err() {
        let error_data = GameActionResponseData::Error {
            error_type: GameActionError::InvalidAction,
        };
        return web::Json(error_data);
    }

    let mut gameplay_manager = state.gameplay_manager.lock().unwrap();
    let player_action_result = gameplay_manager.perform_player_action(
        &payload.game_id,
        &payload.player_token,
        &player_action_param.unwrap(),
    );

    let response_data = match player_action_result {
        Ok(_) => GameActionResponseData::Success {},
        Err(err) => GameActionResponseData::Error { error_type: err },
    };
    return web::Json(response_data);
}
