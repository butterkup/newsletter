use newsletter::configuration::get_configuration;
use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let configuration = get_configuration();
  let listener = std::net::TcpListener::bind(configuration.application.bind_address())?;
  let connection_db = PgPoolOptions::new()
    .acquire_timeout(std::time::Duration::from_secs(5))
    .connect_lazy(&configuration.database.db_url().expose_secret())
    .expect("Failed setting up database pool");
  newsletter::telemetry::setup_subscriber("newsletter".into(), "info".into(), std::io::stdout);
  newsletter::startup::run(listener, connection_db)?.await
}
