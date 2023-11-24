use crate::error_enums::{CreateGameError, GameJoinError};
use std::collections::HashMap;

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
pub enum PlayerAction {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
pub struct GameplayManager {
    pub game_entries: HashMap<String, GameEntry>,
}

impl GameplayManager {
    pub fn register_game(&mut self, game_id: &String) -> Result<(), CreateGameError> {
        if self.game_entries.contains_key(game_id) {
            return Err(CreateGameError::GameAlreadyExist);
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
    ) -> Result<(), GameJoinError> {
        if !self.game_entries.contains_key(game_id) {
            return Err(GameJoinError::InvalidGameId);
        }

        match self.get_mut_player_slot_to_join(game_id, player_token) {
            Some(player_slot) => {
                player_slot.player_token = player_token.to_string();
                player_slot.display_name = player_display_name.to_string();
            }
            None => {
                println!("no free player slot found! {:?}", player_display_name);
                return Err(GameJoinError::GameRoomFull);
            }
        };

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
        };
        None
    }

    fn get_mut_player_slot_to_join(
        &mut self,
        game_id: &String,
        player_token: &String,
    ) -> Option<&mut PlayerInfo> {
        match self.game_entries.get_mut(game_id) {
            Some(game_entry) => {
                if game_entry.player_1.player_token.is_empty()
                    || game_entry.player_1.player_token == player_token.to_string()
                {
                    return Some(&mut game_entry.player_1);
                }
                if game_entry.player_2.player_token.is_empty()
                    || game_entry.player_2.player_token == player_token.to_string()
                {
                    return Some(&mut game_entry.player_2);
                }
            }
            _ => {}
        };
        None
    }
}
