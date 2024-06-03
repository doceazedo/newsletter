use config::{Config, File};
use secrecy::{ExposeSecret, Secret};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub port: u16,
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
            self.user, self.password.expose_secret(), self.host, self.port, self.name
        ).into()
    }

    pub fn get_uri_without_db(&self) -> Secret<String> {
        format!(
            "postgres://{}:{}@{}:{}",
            self.user, self.password.expose_secret(), self.host, self.port
        ).into()
    }
}

pub fn get_config() -> Settings {
    let settings = Config::builder()
        .add_source(File::with_name("config.yaml"))
        .build()
        .expect("Could not load config.json");
    settings
        .try_deserialize::<Settings>()
        .expect("Could not deserialize config")
}
