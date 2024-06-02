use config::{Config, File};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    host: String,
    user: String,
    password: String,
    name: String,
    port: u16,
}

impl DatabaseSettings {
    pub fn get_uri(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.name
        )
    }
}

pub fn get_config() -> Result<Settings, config::ConfigError> {
    let settings = Config::builder()
        .add_source(File::with_name("config.yaml"))
        .build()
        .expect("Could not load config.json");
    settings.try_deserialize::<Settings>()
}
