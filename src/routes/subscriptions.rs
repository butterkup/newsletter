use actix_web::{web, HttpResponse};
use chrono::Utc;
use uuid::Uuid;

#[allow(unused)]
#[derive(serde::Deserialize)]
pub struct User {
  name: String,
  email: String,
}

pub async fn subscribe(form: web::Form<User>, pool: web::Data<sqlx::PgPool>) -> HttpResponse {
  match sqlx::query!(
    r#"
      INSERT INTO subscriptions (id, email, name, subscribed_at)
      VALUES ($1, $2, $3, $4)
    "#,
    Uuid::new_v4(),
    form.email,
    form.name,
    Utc::now().naive_utc()
  )
  .execute(pool.get_ref())
  .await
  {
    Ok(_) => HttpResponse::Ok().finish(),
    Err(error) => {
      dbg!(error);
      HttpResponse::InternalServerError().body("Error creating subscription.")
    }
  }
}
