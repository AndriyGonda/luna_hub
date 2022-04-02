use serde_derive::Deserialize;
use std::io::Result;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Config {
    pub application: AppConfig,
    pub database: DatabaseConfig,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u64,
    pub db: String,
    pub user: String,
    pub password: String,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&content)?)
    }
    pub fn get_database_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.database.user,
            self.database.password,
            self.database.host,
            self.database.port,
            self.database.db
        )
    }
}
