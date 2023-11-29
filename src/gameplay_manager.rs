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
    pub fn join_game(
        &mut self,
        player_token: &String,
        player_display_name: &String,
    ) -> Result<(), GameJoinError> {
        match self.get_mut_player_slot_to_join(player_token) {
            Some(player_slot) => {
                player_slot.player_token = player_token.to_string();
                player_slot.display_name = player_display_name.to_string();
            }
            None => {
                println!("no free player slot found! {:?}", player_display_name);
                return Err(GameJoinError::GameRoomFull);
            }
        };

        if self.both_players_joined() {
            println!("game is ready to play, both players joined!");
            self.player_1.is_current_turn = true;
        }
        return Ok(());
    }

    pub fn perform_player_action(&mut self, player_token: &String, player_action: &PlayerAction) -> Result<(), GameActionError> {
        let player_info_option = self.get_mut_player_info(player_token);

        if let Some(player_info) = player_info_option {
            return if player_info.is_current_turn {
                player_info.current_action = Option::from(player_action.clone());
                player_info.is_current_turn = false;

                if self.both_players_made_current_action() {
                    let player1_action = self.player_1.current_action.as_mut().unwrap().clone();
                    self.player_1.history.push(player1_action);

                    let player2_action = self.player_2.current_action.as_mut().unwrap().clone();
                    self.player_2.history.push(player2_action);

                    // reset action
                    self.player_1.current_action = None;
                    self.player_1.is_current_turn = false;
                    self.player_2.current_action = None;
                    self.player_2.is_current_turn = false;
                } else {
                    self
                        .get_mut_opponent_player(player_token.clone()).unwrap()
                        .is_current_turn = true;
                }

                Ok(())
            } else {
                Err(GameActionError::NotYourTurn)
            }
        }
        return Err(GameActionError::GenericError);
    }

    fn get_mut_player_info(&mut self, player_token: &String) -> Option<&mut PlayerInfo> {
        if self.player_1.player_token == player_token.to_string() {
            return Some(&mut self.player_1);
        }
        if self.player_2.player_token == player_token.to_string() {
            return Some(&mut self.player_2);
        }
        return None;
    }

    fn both_players_made_current_action(&self) -> bool {
        self.player_1.current_action.is_some() && self.player_2.current_action.is_some()
    }

    fn both_players_joined(&self) -> bool {
        !self.player_1.player_token.is_empty() && !self.player_2.player_token.is_empty()
    }

    fn get_mut_opponent_player(&mut self, player_token: String) -> Option<&mut PlayerInfo> {
        if self.player_1.player_token == player_token {
            return Some(&mut self.player_2);
        }

        if self.player_2.player_token == player_token {
            return Some(&mut self.player_1);
        }
        return None;
    }

    fn get_mut_player_slot_to_join(
        &mut self,
        player_token: &String,
    ) -> Option<&mut PlayerInfo> {
        if self.player_1.player_token.is_empty()
            || self.player_1.player_token == player_token.to_string()
        {
            return Some(&mut self.player_1);
        }
        if self.player_2.player_token.is_empty()
            || self.player_2.player_token == player_token.to_string()
        {
            return Some(&mut self.player_2);
        }

        return None;
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
            println!("game_entries {:?}", self.game_entries);
            return res;
        }

        return Err(GameActionError::InvalidGameId);
    }
}
