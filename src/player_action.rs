use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum PlayerAction {
    Rock,
    Paper,
    Scissors,
}

pub trait Beats {
    fn beats(&self) -> Self;
}

impl Beats for PlayerAction {
    fn beats(&self) -> PlayerAction {
        // match is exhaustive, so every enum variant must be covered
        match *self {
            PlayerAction::Rock => PlayerAction::Scissors,
            PlayerAction::Paper => PlayerAction::Rock,
            PlayerAction::Scissors => PlayerAction::Paper,
        }
    }
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
