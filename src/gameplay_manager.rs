use crate::error_enums::{CreateGameError, GameActionError, GameJoinError};
use crate::game_entry::{GameEntry, PlayerInfo};
use crate::player_action::PlayerAction;
use std::collections::HashMap;

#[derive(Debug)]
pub struct GameplayManager {
    pub game_entries: HashMap<String, GameEntry>,
}

impl Default for GameplayManager {
    fn default() -> Self {
        Self {
            game_entries: HashMap::new(),
        }
    }
}

impl GameplayManager {
    pub fn register_game(&mut self, game_id: &String) -> Result<(), CreateGameError> {
        if self.game_entries.contains_key(game_id) {
            return Err(CreateGameError::GameAlreadyExist);
        }

        let key = game_id.clone();
        let game_entry = GameEntry {
            player_1: PlayerInfo::default(),
            player_2: PlayerInfo::default(),
        };
        self.game_entries.insert(key, game_entry);
        return Ok(());
    }

    pub fn join_game(
        &mut self,
        game_id: &String,
        player_token: &String,
        player_display_name: &String,
    ) -> Result<(), GameJoinError> {
        let game_entry = self.game_entries.get_mut(game_id);
        if let Some(val) = game_entry {
            return val.join_game(player_token, player_display_name);
        }

        return Err(GameJoinError::InvalidGameId);
    }

    pub fn perform_player_action(
        &mut self,
        game_id: &String,
        player_token: &String,
        player_action: &PlayerAction,
    ) -> Result<(), GameActionError> {
        let game_entry = self.game_entries.get_mut(game_id);
        if let Some(val) = game_entry {
            let res = val.perform_player_action(player_token, player_action);
            // println!("game_entries {:?}", self.game_entries);
            return res;
        }

        return Err(GameActionError::InvalidGameId);
    }

    pub fn gameplay_data(&self) -> HashMap<String, GameEntry> {
        return self.game_entries.clone();
    }
}
