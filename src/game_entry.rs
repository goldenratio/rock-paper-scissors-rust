use crate::error_enums::{GameActionError, GameJoinError};
use crate::player_action::PlayerAction;
use serde::Serialize;

#[derive(Debug, Clone, Default, Serialize)]
pub struct PlayerInfo {
    pub display_name: String,
    pub player_token: String,
    pub history: Vec<PlayerAction>,
    pub current_action: Option<PlayerAction>,
    pub score: i16,
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
        match self.get_player_info_to_join_mut(player_token) {
            Some(player_info) => {
                player_info.player_token = player_token.to_string();
                player_info.display_name = player_display_name.to_string();
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

    pub fn perform_player_action(
        &mut self,
        player_token: &String,
        player_action: &PlayerAction,
    ) -> Result<(), GameActionError> {
        let player_info_option = self.get_player_info_mut(player_token);

        if let Some(player_info) = player_info_option {
            if player_info.current_action.is_some() {
                return Err(GameActionError::ActionAlreadyPerformed);
            }

            player_info.current_action = Option::from(player_action.clone());

            if self.both_players_made_current_action() {
                let player_1 = &mut self.player_1;
                let player1_action = player_1
                    .current_action
                    .take()
                    .expect("No current action on player_1");

                let mut player_2 = &mut self.player_2;
                let player2_action = player_2
                    .current_action
                    .take()
                    .expect("No current action on player_2");

                player_1.history.push(player1_action.clone());
                player_2.history.push(player2_action.clone());

                // reset action
                self.player_1.current_action = None;
                self.player_2.current_action = None;

                self.update_result(player1_action.clone(), player2_action.clone());
            }

            return Ok(());
        }
        return Err(GameActionError::GenericError);
    }

    fn update_result(&self, player_1_action: PlayerAction, player_2_action: PlayerAction) {
        println!(
            "update result (P1){:?} and (P2){:?}",
            player_1_action, player_2_action
        );
    }

    fn both_players_made_current_action(&self) -> bool {
        return self.player_1.current_action.is_some() && self.player_2.current_action.is_some();
    }

    fn both_players_joined(&self) -> bool {
        return !self.player_1.player_token.is_empty() && !self.player_2.player_token.is_empty();
    }

    fn get_player_info_mut(&mut self, player_token: &String) -> Option<&mut PlayerInfo> {
        if self.player_1.player_token == player_token.to_string() {
            return Some(&mut self.player_1);
        }
        if self.player_2.player_token == player_token.to_string() {
            return Some(&mut self.player_2);
        }
        return None;
    }

    fn get_player_info_to_join_mut(&mut self, player_token: &String) -> Option<&mut PlayerInfo> {
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
