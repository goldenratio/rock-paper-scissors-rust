use std::str::FromStr;
use serde::Deserialize;

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
