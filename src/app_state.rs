use crate::game_creator::GameCreator;
use crate::gameplay_manager::GameplayManager;
use std::sync::Mutex;

#[derive(Debug)]
pub struct AppState {
    pub game_creator: Mutex<GameCreator>,
    pub gameplay_manager: Mutex<GameplayManager>,
    // pub server_handle: Mutex<Option<ServerHandle>>
}
