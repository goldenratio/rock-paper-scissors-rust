use crate::error_enums::{CreateGameError, GameActionError, GameJoinError};
use serde::Deserialize;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
pub struct PlayerInfo {
    pub display_name: String,
    pub player_token: String,
    pub history: Vec<PlayerAction>,
    pub current_action: Option<PlayerAction>,
    pub is_current_turn: bool,
}

#[derive(Debug)]
pub struct GameEntry {
    pub player_1: PlayerInfo,
    pub player_2: PlayerInfo,
}

impl GameEntry {
    pub fn both_players_joined(&self) -> bool {
        !self.player_1.player_token.is_empty() && !self.player_2.player_token.is_empty()
    }

    pub fn get_mut_opponent_player(&mut self, player_token: String) -> &mut PlayerInfo {
        if self.player_1.player_token == player_token {
            return &mut self.player_1;
        }
        return &mut self.player_2;
    }
}

#[derive(Clone, Debug, Deserialize)]
pub enum PlayerAction {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for PlayerAction {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "rock" => Ok(PlayerAction::Rock),
            "paper" => Ok(PlayerAction::Paper),
            "scissors" => Ok(PlayerAction::Scissors),
            _ => Err(()),
        }
    }
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
                history: vec![],
                current_action: None,
                is_current_turn: false,
            },
            player_2: PlayerInfo {
                display_name: "".to_string(),
                player_token: "".to_string(),
                history: vec![],
                current_action: None,
                is_current_turn: false,
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
        let game_entry = self.game_entries.get_mut(game_id);
        let both_players_joined = match &game_entry {
            Some(game_entry) => game_entry.both_players_joined(),
            None => false,
        };

        if both_players_joined {
            println!("game is ready to play, both players joined!");
            game_entry.unwrap().player_1.is_current_turn = true;
        }
        return Ok(());
    }

    pub fn perform_player_action(
        &mut self,
        game_id: &String,
        player_token: &String,
        player_action: &PlayerAction,
    ) -> Result<(), GameActionError> {
        println!("perform player action {:?}", player_action);
        let player_info_option = self.get_mut_player_info(game_id, player_token);

        if let Some(player_info) = player_info_option {
            if player_info.is_current_turn {
                player_info.current_action = Option::from(player_action.clone());
                player_info.is_current_turn = false;

                let game_entry = self.game_entries.get_mut(game_id).unwrap();
                game_entry
                    .get_mut_opponent_player(player_token.clone())
                    .is_current_turn = true;

                return Ok(());
            } else {
                return Err(GameActionError::NotYourTurn);
            }
        }
        return Err(GameActionError::InvalidGameId);
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
