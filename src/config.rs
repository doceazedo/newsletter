use config::{Config, Environment, File};
use secrecy::{ExposeSecret, Secret};
use serde_aux::prelude::{deserialize_bool_from_anything, deserialize_number_from_string};
use sqlx::postgres::{PgConnectOptions, PgSslMode};

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
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub ssl: bool,
}

impl DatabaseSettings {
    pub fn get_options(&self) -> PgConnectOptions {
        self.get_options_without_default_db().database(&self.name)
    }

    pub fn get_options_without_default_db(&self) -> PgConnectOptions {
        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.user)
            .password(self.password.expose_secret())
            .port(self.port)
            .ssl_mode(if self.ssl {
                PgSslMode::Require
            } else {
                PgSslMode::Prefer
            })
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
