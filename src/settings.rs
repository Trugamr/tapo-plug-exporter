use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub tapo: Tapo,
}

#[derive(Debug, Deserialize)]
pub struct Tapo {
    pub username: String,
    pub password: String,
    pub ip: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name("config.yml").required(false))
            .add_source(Environment::default().separator("_"))
            .build()?;

        config.try_deserialize()
    }
}
