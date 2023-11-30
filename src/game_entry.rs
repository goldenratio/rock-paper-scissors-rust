use serde::Serialize;
use crate::error_enums::{GameActionError, GameJoinError};
use crate::player_action::PlayerAction;

#[derive(Debug, Clone, Default, Serialize)]
pub struct PlayerInfo {
    pub display_name: String,
    pub player_token: String,
    pub history: Vec<PlayerAction>,
    pub current_action: Option<PlayerAction>
}

#[derive(Debug)]
pub enum GameResult {
    Win,
    Lose,
    Draw,
}

#[derive(Debug, Clone, Serialize)]
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
        }
        return Ok(());
    }

    pub fn perform_player_action(&mut self, player_token: &String, player_action: &PlayerAction) -> Result<(), GameActionError> {
        let player_info_option = self.get_mut_player_info(player_token);

        if let Some(player_info) = player_info_option {
            player_info.current_action = Option::from(player_action.clone());

            if self.both_players_made_current_action() {
                let player1_action = self.player_1.current_action.as_mut().unwrap().clone();
                self.player_1.history.push(player1_action);

                let player2_action = self.player_2.current_action.as_mut().unwrap().clone();
                self.player_2.history.push(player2_action);

                // println!("player_1 {:?} player_2 {:?}", player1_action, player2_action);

                // reset action
                self.player_1.current_action = None;
                self.player_2.current_action = None;
            }

            return Ok(());
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
