use std::string::ParseError;

#[derive(Debug)]
pub struct GameCreator {
    pub game_id: i32,
    pub player_token_id: i32,
}

impl GameCreator {
    pub fn create_new_game(&mut self) -> Result<String, ParseError> {
        self.game_id += 1;
        // todo: hash the gameId
        let res = format!("game{}", self.game_id);
        Ok(res)
    }

    pub fn create_new_player_token(&mut self) -> Result<String, ParseError> {
        self.player_token_id += 1;
        // todo: hash the token
        let res = format!("player_{}_token_{}", self.game_id, self.player_token_id);
        Ok(res)
    }
}
