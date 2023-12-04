use crate::game_creator::GameCreator;
use crate::gameplay_manager::GameplayManager;
use crate::server_settings::ServerSettings;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug)]
pub struct AdminInfo {
    pub admin_jwt_secret: String,
    pub admin_username: String,
    pub admin_password: String,
}

#[derive(Debug)]
pub struct AppState {
    pub game_creator: Mutex<GameCreator>,
    pub gameplay_manager: Mutex<GameplayManager>,
    pub admin_info: Mutex<AdminInfo>,
}

impl AppState {
    pub fn new(settings: &ServerSettings) -> AppState {
        AppState {
            game_creator: Mutex::new(GameCreator {
                game_id: 0,
                player_token_id: 0,
            }),
            gameplay_manager: Mutex::new(GameplayManager {
                game_entries: HashMap::new(),
            }),
            admin_info: Mutex::new(AdminInfo {
                admin_jwt_secret: settings.admin_jwt_secret.clone(),
                admin_username: settings.admin_username.clone(),
                admin_password: settings.admin_password.clone(),
            }),
        }
    }
}
