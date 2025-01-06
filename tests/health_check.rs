use secrecy::ExposeSecret;
use sqlx::{Connection, Executor};

#[actix_web::test]
async fn health() {
  let app = TestApp::new().await;
  let client = reqwest::Client::new();
  let resp = client
    .get(format!("{}/health_check", app.base_url))
    .send()
    .await
    .expect("Health check request failed");
  assert!(resp.status().is_success());
  assert_eq!(resp.content_length(), Some(0));
}

#[actix_web::test]
async fn subscribe_returns_200_for_valid_form_data() {
  let app = TestApp::new().await;
  let client = reqwest::Client::new();
  let resp = client
    .post(format!("{}/subscriptions", app.base_url))
    .header("Content-Type", "application/x-www-form-urlencoded")
    .body("name=Simon%20Nganga&email=theedushbag%40gmail.com")
    .send()
    .await
    .expect("Subscription request failed");
  let _saved = sqlx::query!("SELECT email, name FROM subscriptions",)
    .fetch_one(&app.pool)
    .await
    .expect("Failed to fatch saved subscription");
  assert_eq!(resp.status().as_u16(), 200);
}

#[actix_web::test]
async fn subscribe_returns_400_when_data_is_missing() {
  let app = TestApp::new().await;
  let invalid_data = vec![
    ("name=John%20Doe", "Missing email"),
    ("email=johndoe%40gmail.com", "Missing name"),
    ("", "Missing email and name"),
  ];
  let client = reqwest::Client::new();

  for (invalid_data, error_msg) in invalid_data {
    let resp = client
      .post(format!("{}/subscriptions", app.base_url))
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

static TRACING: std::sync::LazyLock<()> = std::sync::LazyLock::new(|| {
  let mode = "testing".into();
  let level = "debug".into();
  if std::env::var("TEST_LOG")
    .is_ok_and(|value| matches!(value.to_lowercase().as_str(), "show" | "true" | "yes" | "y"))
  {
    newsletter::telemetry::setup_subscriber(mode, level, std::io::stdout);
  } else {
    newsletter::telemetry::setup_subscriber(mode, level, std::io::sink);
  }
});

pub struct TestApp {
  pub base_url: String,
  pub pool: sqlx::PgPool,
  pub conf: newsletter::configuration::Settings,
}

impl TestApp {
  pub async fn new() -> TestApp {
    std::sync::LazyLock::force(&TRACING);
    let conf = {
      let mut conf = newsletter::configuration::get_configuration();
      conf.database.dbname = uuid::Uuid::new_v4().to_string();
      conf
    };
    let listener = std::net::TcpListener::bind(conf.application.bind_address())
      .expect("Failed finding setting up tcp listener");
    let port = listener
      .local_addr()
      .expect("Failed unwrapping assigned port")
      .port();
    let pool = Self::setup_test_db(&conf.database).await;
    let server =
      newsletter::startup::run(listener, pool.clone()).expect("Test server startup failed");
    let _ = tokio::spawn(server);
    Self {
      base_url: format!("http://{}:{}", conf.application.host, port),
      pool,
      conf,
    }
  }

  pub async fn setup_test_db(conf: &newsletter::configuration::DatabaseSettings) -> sqlx::PgPool {
    let mut conn = sqlx::PgConnection::connect(conf.db_url_unnamed().expose_secret())
      .await
      .expect("Postgres won't connect");
    conn
      .execute(format!(r#"CREATE DATABASE "{}";"#, conf.dbname).as_str())
      .await
      .expect("Failed creating test database");
    conn.close().await.expect("closing PgConnection failed");
    let pool = sqlx::PgPool::connect(conf.db_url().expose_secret())
      .await
      .expect("failed connecting PgPool for testing");
    sqlx::migrate!("./migrations")
      .run(&pool)
      .await
      .expect("Failed migrating test database");
    pool
  }
}
