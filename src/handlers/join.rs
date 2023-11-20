use crate::enum_types::GameJoinErrorType;
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
enum GameJoinResponseData {
    #[serde(rename = "data")]
    Success {
        #[serde(rename = "playerToken")]
        player_token: String,
    },
    #[serde(rename = "data")]
    Error {
        #[serde(rename = "error")]
        error_type: GameJoinErrorType,
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
    let mut gameplay_manager = state.gameplay_manager.lock().unwrap();

    let response_data = match game_creator.create_new_player_token() {
        Ok(player_token) => {
            match gameplay_manager.join_game(&payload.game_id, &player_token, &payload.player_name)
            {
                Ok(_) => GameJoinResponseData::Success { player_token },
                Err(error_type) => GameJoinResponseData::Error { error_type },
            }
        }
        Err(_) => GameJoinResponseData::Error {
            error_type: GameJoinErrorType::GenericError,
        },
    };

    return web::Json(response_data);
}
