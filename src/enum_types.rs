use serde::Serialize;

#[derive(Debug)]
pub enum PlayerAction {
    Rock,
    Paper,
    Scissors,
}

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
    PlayerAlreadyJoined,
    GenericError,
}
