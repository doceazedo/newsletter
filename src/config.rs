use config::{Config, File};
use secrecy::{ExposeSecret, Secret};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub ip: String,
    pub port: u16,
    pub database: DatabaseSettings,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub host: String,
    pub user: String,
    pub password: Secret<String>,
    pub name: String,
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
    let config_file = match std::env::var("ENVIRONMENT") {
        Ok(env) => format!("config.{}.yaml", env),
        Err(_) => "config.yaml".to_string(),
    };
    let settings = Config::builder()
        .add_source(File::with_name(&config_file))
        .build()
        .unwrap_or_else(|_| panic!("Could not load {}", config_file));
    settings
        .try_deserialize::<Settings>()
        .expect("Could not deserialize config")
}
