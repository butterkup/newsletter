#[actix_web::test]
async fn health() {
  let base = spawn_app();
  let client = reqwest::Client::new();
  let resp = client
    .get(format!("{}/health_check", base))
    .send()
    .await
    .expect("Health check request failed");
  assert!(resp.status().is_success());
  assert_eq!(resp.content_length(), Some(0));
}

#[actix_web::test]
async fn subscribe_returns_200_for_valid_form_data() {
  let base = spawn_app();
  let client = reqwest::Client::new();
  let resp = client
    .post(format!("{}/subscriptions", base))
    .header("Content-Type", "application/x-www-form-urlencoded")
    .body("name=Simon%20Nganga&email=theedushbag%40gmail.com")
    .send()
    .await
    .expect("Subscription request failed");
  assert_eq!(resp.status().as_u16(), 200);
}

#[actix_web::test]
async fn subscribe_returns_400_when_data_is_missing() {
  let base = spawn_app();
  let invalid_data = vec![
    ("name=John%20Doe", "Missing email"),
    ("email=johndoe%40gmail.com", "Missing name"),
    ("", "Missing email and name"),
  ];
  let client = reqwest::Client::new();

  for (invalid_data, error_msg) in invalid_data {
    let resp = client
      .post(format!("{}/subscriptions", base))
      .header("Content-Type", "application/x-www-form-urlencoded")
      .body(invalid_data)
      .send()
      .await
      .expect("Subscription request failed");
    assert_eq!(
      resp.status().as_u16(),
      400,
      "Expected a failure with status 400 on malformed payload {{ message={:?}, data={:?} }}",
      error_msg,
      invalid_data
    );
  }
}

fn spawn_app() -> String {
  let listener = std::net::TcpListener::bind("localhost:0").expect("Failed finding random port");
  let port = listener
    .local_addr()
    .expect("Failed unwrapping port")
    .port();
  let server = newsletter::startup::run(listener).expect("Failed binding address");
  let _ = tokio::spawn(server);
  format!("http://localhost:{}", port)
}
