#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let listener = std::net::TcpListener::bind("localhost:8080")?;
  newsletter::startup::run(listener)?.await
}
