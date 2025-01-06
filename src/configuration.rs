use secrecy::ExposeSecret;

enum Environment {
  Production,
  Development,
}

impl Environment {
  fn as_str(&self) -> &'static str {
    match self {
      Self::Development => "development",
      Self::Production => "production",
    }
  }
}

impl From<&str> for Environment {
  fn from(value: &str) -> Self {
    match value {
      "development" => Self::Development,
      "production" => Self::Production,
      _ => panic!("Invalid Environment: {value}"),
    }
  }
}

#[derive(serde::Deserialize, Clone)]
pub struct Settings {
  pub database: DatabaseSettings,
  pub application: ApplicationSettings,
}

#[derive(serde::Deserialize, Clone)]
pub struct DatabaseSettings {
  pub username: String,
  pub dbname: String,
  pub port: u16,
  pub host: String,
  pub password: secrecy::SecretString,
}

#[derive(serde::Deserialize, Clone)]
pub struct ApplicationSettings {
  pub host: String,
  pub port: u16,
}

impl ApplicationSettings {
  pub fn bind_address(&self) -> String {
    format!("{}:{}", self.host, self.port)
  }
}

pub fn try_get_configuration() -> Result<Settings, config::ConfigError> {
  let appenv = Environment::from(
    std::env::var("APP_ENV")
      .expect("Must provide environment for application to run by setting `APP_ENV`")
      .as_str(),
  );
  let config_dir = std::env::current_dir().unwrap().join("configuration");

  dbg!(&config_dir);

  config::Config::builder()
    .add_source(config::File::from(config_dir.join("default")).required(true))
    .add_source(config::File::from(config_dir.join(appenv.as_str())).required(false))
    .add_source(config::File::from(config_dir.join("local")).required(false))
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
      self.username,
      self.password.expose_secret(),
      self.host,
      self.port
    )
    .into()
  }

  pub fn db_url_named(&self, name: &str) -> secrecy::SecretString {
    format!(
      "postgres://{}:{}@{}:{}/{}",
      self.username,
      self.password.expose_secret(),
      self.host,
      self.port,
      name
    )
    .into()
  }
}
