use actix_web::{web, App, HttpServer};

pub fn run(listener: std::net::TcpListener) -> Result<actix_web::dev::Server, std::io::Error> {
  let server = HttpServer::new(|| {
    App::new()
      .route("/health_check", web::get().to(crate::routes::health_check))
      .route("/subscriptions", web::post().to(crate::routes::subscribe))
  })
  .listen(listener)?
  .run();
  Ok(server)
}
