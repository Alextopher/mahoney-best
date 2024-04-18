use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub username: &'static str,
    pub password: &'static str,
}

impl Config {
    pub fn load() -> Config {
        Config {
            username: "admin",
            password: "password",
        }
    }

    pub fn check_admin(&self, username: &str, password: &str) -> bool {
        (self.username, self.password) == (username, password)
    }
}
