use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Debug, Serialize, Deserialize)]
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
