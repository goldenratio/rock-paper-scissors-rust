use sha256::digest;
use std::string::ParseError;

#[derive(Debug)]
pub struct GameCreator {
    pub game_id: i32,
    pub player_token_id: i32,
}

impl Default for GameCreator {
    fn default() -> Self {
        Self {
            game_id: 0,
            player_token_id: 0,
        }
    }
}

impl GameCreator {
    pub fn create_new_game(&mut self) -> Result<String, ParseError> {
        self.game_id += 1;
        let res = format!("game{}", self.game_id);
        let res_hash = digest(res)[..6].to_string();
        return Ok(res_hash);
    }

    pub fn create_new_player_token(&mut self) -> Result<String, ParseError> {
        self.player_token_id += 1;
        let res = format!("{}{}", self.game_id, self.player_token_id);
        let res_hash = digest(res);
        return Ok(res_hash);
    }
}
