#[actix_web::test]
async fn health() {
  let _handle = spawn_app();
  let client = reqwest::Client::new();
  let resp = client
    .get("http://localhost:8080/health_check")
    .send()
    .await
    .expect("Health check request failed");
  assert!(resp.status().is_success());
  assert_eq!(resp.content_length(), Some(0));
}

fn spawn_app() -> tokio::task::JoinHandle<Result<(), std::io::Error>> {
  let server = newsletter::run().expect("Failed binding address");
  tokio::spawn(server)
}
