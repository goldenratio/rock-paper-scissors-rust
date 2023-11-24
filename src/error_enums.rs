use serde::Serialize;

#[derive(Serialize, Debug)]
pub enum CreateGameErrorType {
    GameAlreadyExist,
    GenericError,
}

#[derive(Serialize)]
pub enum GameActionErrorType {
    InvalidGameId,
    NotYourTurn,
    GenericError,
}

#[derive(Serialize)]
pub enum GameJoinErrorType {
    InvalidGameId,
    GameRoomFull,
    GenericError,
}
