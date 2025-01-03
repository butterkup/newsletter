use secrecy::ExposeSecret;

#[derive(serde::Deserialize, Clone)]
pub struct Settings {
  pub database: DatabaseSettings,
  pub port: u16,
}

#[derive(serde::Deserialize, Clone)]
pub struct DatabaseSettings {
  pub username: String,
  pub dbname: String,
  pub port: u16,
  pub host: String,
  pub password: secrecy::SecretString,
}

pub fn try_get_configuration() -> Result<Settings, config::ConfigError> {
  config::Config::builder()
    .add_source(config::File::with_name("configuration"))
    .build()?
    .try_deserialize()
}

pub fn get_configuration() -> Settings {
  try_get_configuration().expect("Failed loading configurations")
}

impl DatabaseSettings {
  pub fn db_url(&self) -> secrecy::SecretString {
    self.db_url_named(&self.dbname)
  }

  pub fn db_url_unnamed(&self) -> secrecy::SecretString {
    format!(
      "postgres://{}:{}@{}:{}",
      self.username, self.password.expose_secret(), self.host, self.port
    ).into()
  }

  pub fn db_url_named(&self, name: &str) -> secrecy::SecretString {
    format!(
      "postgres://{}:{}@{}:{}/{}",
      self.username, self.password.expose_secret(), self.host, self.port, name
    ).into()
  }
}
