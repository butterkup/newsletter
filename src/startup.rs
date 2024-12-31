use actix_web::{web, App, HttpServer};

pub fn run(
  listener: std::net::TcpListener,
  connection: sqlx::PgPool,
) -> Result<actix_web::dev::Server, std::io::Error> {
  let connection = web::Data::new(connection);
  let server = HttpServer::new(move || {
    App::new()
      .route("/health_check", web::get().to(crate::routes::health_check))
      .route("/subscriptions", web::post().to(crate::routes::subscribe))
      .app_data(connection.clone())
  })
  .listen(listener)?
  .run();
  Ok(server)
}
