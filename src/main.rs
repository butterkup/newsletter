use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
  let name = req.match_info().get("name").unwrap_or("World");
  format!("Hello {}!\n", name)
}

async fn health_check() -> impl Responder {
  HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
      .route("/greet/", web::get().to(greet))
      .route("/greet/{name}", web::get().to(greet))
      .route("/health_check", web::get().to(health_check))
  })
  .bind(("localhost", 8080))?
  .run()
  .await
}
