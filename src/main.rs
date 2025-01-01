use newsletter::configuration::get_configuration;
use sqlx::PgPool;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use tracing_log::LogTracer;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let configuration = get_configuration();
  let address = format!("127.0.0.1:{}", configuration.port);
  let listener = std::net::TcpListener::bind(address)?;
  let connection_db = PgPool::connect(&configuration.database.db_url())
    .await
    .expect("Failed connecting to database");
  // env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

  LogTracer::init().expect("Failed to set logger");
  let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
  let formatting_layer = BunyanFormattingLayer::new("newsletter".into(), std::io::stdout);
  let subscriber = Registry::default()
    .with(env_filter)
    .with(JsonStorageLayer)
    .with(formatting_layer);
  set_global_default(subscriber).expect("Failed to set subscriber");

  newsletter::startup::run(listener, connection_db)?.await
}
