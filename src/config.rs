#[derive(serde::Deserialize)]
pub struct DBConfig {
    pub user: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub name: String,
}

impl DBConfig {
    pub fn conn_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.name
        )
    }
}

#[derive(serde::Deserialize)]
pub struct Config {
    pub app_port: u16,
    pub db: DBConfig,
}

pub fn get_config() -> Result<Config, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::new("config.yml", config::FileFormat::Yaml))
        .build()?;
    settings.try_deserialize()
}
