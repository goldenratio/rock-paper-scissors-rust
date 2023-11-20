use std::collections::HashMap;
use std::string::ParseError;
use std::sync::Mutex;

#[derive(Debug)]
pub enum PlayerAction {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
pub struct PlayerInfo {
    pub display_name: String,
    pub player_token: String,
    pub actions: Vec<PlayerAction>,
}

#[derive(Debug)]
pub struct GameEntry {
    pub player_1: PlayerInfo,
    pub player_2: PlayerInfo,
}

#[derive(Debug)]
pub struct AppState {
    pub game_creator: Mutex<GameCreator>,
    pub game_entries: Mutex<HashMap<String, GameEntry>>,
}

#[derive(Debug)]
pub struct GameCreator {
    pub game_id: i32,
    pub player_token_id: i32,
}

impl GameCreator {
    pub fn create_new_game(&mut self) -> Result<String, ParseError> {
        self.game_id += 1;
        let res = format!("game{}", self.game_id);
        Ok(res)
    }

    pub fn create_new_player_token(&mut self) -> Result<String, ParseError> {
        self.player_token_id += 1;
        let res = format!("player_token_{}", self.player_token_id);
        Ok(res)
    }
}
