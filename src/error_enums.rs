use serde::Serialize;

#[derive(Serialize, Debug)]
pub enum CreateGameError {
    GameAlreadyExist,
    GenericError,
}

#[derive(Serialize)]
pub enum GameActionError {
    InvalidGameId,
    NotYourTurn,
    GenericError,
}

#[derive(Serialize)]
pub enum GameJoinError {
    InvalidGameId,
    GameRoomFull,
    GenericError,
}
