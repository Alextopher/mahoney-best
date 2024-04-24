use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub username: String,
    pub password: String,
}

impl Config {
    pub fn load() -> Config {
        let config = std::fs::read_to_string("config.toml").expect("Could not open config.toml");
        toml::from_str(&config).expect("Failed to parse config.toml")
    }

    pub fn check_admin(&self, username: &str, password: &str) -> bool {
        username == self.username && password == self.password
    }
}
