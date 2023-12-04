use config::Config;
use std::collections::HashMap;
use std::net::SocketAddr;

pub struct ServerSettings {
    pub admin_client_route: String,
    pub admin_username: String,
    pub admin_password: String,
    pub admin_jwt_secret: String,
    pub server_port: u16,
    pub use_server_ip_v6: bool,
}

fn get_config_settings() -> HashMap<String, String> {
    let settings = Config::builder()
        .add_source(config::File::with_name("src/config"))
        .build()
        .unwrap()
        .try_deserialize::<HashMap<String, String>>()
        .unwrap();
    return settings;
}

impl Default for ServerSettings {
    fn default() -> Self {
        let settings = get_config_settings();
        Self {
            admin_client_route: settings.get("admin_client_route").unwrap().to_string(),
            admin_username: settings.get("admin_username").unwrap().to_string(),
            admin_password: settings.get("admin_password").unwrap().to_string(),
            admin_jwt_secret: settings.get("admin_jwt_secret").unwrap().to_string(),
            server_port: settings.get("server_port").unwrap().parse::<u16>().unwrap(),
            use_server_ip_v6: settings
                .get("use_server_ip_v6")
                .unwrap()
                .parse::<bool>()
                .unwrap(),
        }
    }
}

impl ServerSettings {
    pub fn get_socket_addr(&self) -> SocketAddr {
        let server_port = self.server_port;
        let use_ip_v6 = self.use_server_ip_v6;
        let ip_addr = if use_ip_v6 {
            "::".to_string()
        } else {
            "127.0.0.1".to_string()
        };

        let socket_addr = SocketAddr::new(ip_addr.parse().unwrap(), server_port);
        return socket_addr;
    }
}
