use config::{Config, Environment, File};
use secrecy::{ExposeSecret, Secret};
use serde_aux::prelude::deserialize_number_from_string;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub ip: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub database: DatabaseSettings,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub host: String,
    pub user: String,
    pub password: Secret<String>,
    pub name: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
}

impl DatabaseSettings {
    pub fn get_uri(&self) -> Secret<String> {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.name
        )
        .into()
    }

    pub fn get_uri_without_db(&self) -> Secret<String> {
        format!(
            "postgres://{}:{}@{}:{}",
            self.user,
            self.password.expose_secret(),
            self.host,
            self.port
        )
        .into()
    }
}

pub fn get_config() -> Settings {
    let settings = Config::builder()
        .add_source(File::with_name("config.yaml"))
        .add_source(Environment::default().separator("_"))
        .build()
        .expect("Could not load config.yml");
    settings
        .try_deserialize::<Settings>()
        .expect("Could not deserialize config")
}
