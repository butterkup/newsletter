use newsletter::configuration::get_configuration;
use sqlx::PgPool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let configuration = get_configuration();
  let address = format!("127.0.0.1:{}", configuration.port);
  let listener = std::net::TcpListener::bind(address)?;
  let connection_db = PgPool::connect(&configuration.database.db_url())
    .await
    .expect("Failed connecting to database");
  newsletter::startup::run(listener, connection_db)?.await
}
