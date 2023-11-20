use crate::enum_types::{CreateGameErrorType, GameJoinErrorType, PlayerAction};
use std::collections::HashMap;
use std::string::ParseError;
use std::string::String;
use std::sync::Mutex;

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
    pub gameplay_manager: Mutex<GameplayManager>,
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

#[derive(Debug)]
pub struct GameplayManager {
    pub game_entries: HashMap<String, GameEntry>,
}

impl GameplayManager {
    pub fn register_game(&mut self, game_id: &String) -> Result<(), CreateGameErrorType> {
        if self.game_entries.contains_key(game_id) {
            return Err(CreateGameErrorType::GameAlreadyExist);
        }

        let key = game_id.clone();
        let game_entry = GameEntry {
            player_1: PlayerInfo {
                display_name: "".to_string(),
                player_token: "".to_string(),
                actions: vec![],
            },
            player_2: PlayerInfo {
                display_name: "".to_string(),
                player_token: "".to_string(),
                actions: vec![],
            },
        };
        self.game_entries.insert(key, game_entry);
        Ok(())
    }

    pub fn join_game(
        &mut self,
        game_id: &String,
        player_token: &String,
        player_display_name: &String,
    ) -> Result<(), GameJoinErrorType> {
        match self.game_entries.get_mut(game_id) {
            Some(game_entry) => {
                if game_entry.player_1.player_token.is_empty()
                    && game_entry.player_1.player_token != game_entry.player_2.player_token
                {
                    game_entry.player_1.player_token = player_token.to_string();
                    game_entry.player_1.display_name = player_display_name.to_string();
                } else if game_entry.player_2.player_token.is_empty()
                    && game_entry.player_2.player_token != game_entry.player_1.player_token
                {
                    game_entry.player_2.player_token = player_token.to_string();
                    game_entry.player_2.display_name = player_display_name.to_string();
                } else {
                    // no player slot available
                    println!("player already joined! {:?}", player_display_name);
                    // TODO: add reconnect
                    return Err(GameJoinErrorType::PlayerAlreadyJoined);
                }
            }
            None => {
                return Err(GameJoinErrorType::InvalidGameId);
            }
        }
        println!("game entries {:?}", self.game_entries);
        Ok(())
    }

    pub fn perform_player_action(
        &mut self,
        game_id: &String,
        player_token: &String,
        player_action: PlayerAction,
    ) {
        //
    }

    fn get_mut_player_info(
        &mut self,
        game_id: &String,
        player_token: &String,
    ) -> Option<&mut PlayerInfo> {
        match self.game_entries.get_mut(game_id) {
            Some(game_entry) => {
                if game_entry.player_1.player_token == player_token.to_string() {
                    return Some(&mut game_entry.player_1);
                }
                if game_entry.player_2.player_token == player_token.to_string() {
                    return Some(&mut game_entry.player_2);
                }
            }
            _ => {}
        }
        None
    }
}
