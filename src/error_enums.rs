use serde::Serialize;

#[derive(Serialize, Debug)]
pub enum CreateGameError {
    GameAlreadyExist,
    GenericError,
}

#[derive(Serialize)]
pub enum GameActionError {
    InvalidGameId,
    InvalidAction,
    NotYourTurn,
    GenericError,
}

#[derive(Serialize)]
pub enum GameJoinError {
    InvalidGameId,
    GameRoomFull,
    GenericError,
}

#[derive(Debug, Serialize)]
pub enum GameplayInfoError {
    GenericError,
}

#[derive(Serialize)]
pub enum AppError {
    BadClientData,
    GenericError,
}

#[derive(Serialize)]
pub enum AppErrorData {
    #[serde(rename = "data")]
    Error {
        #[serde(rename = "error")]
        error_type: AppError,
    },
}
