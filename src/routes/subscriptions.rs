use actix_web::{web, HttpResponse};
use uuid::Uuid;

#[allow(unused)]
#[derive(serde::Deserialize)]
pub struct User {
  name: String,
  email: String,
}

#[tracing::instrument(
  name = "Adding new subscriber",
  skip(pool, form),
  fields(
    subscriber_name = %form.name,
    subscriber_email = %form.email
  )
)]
pub async fn subscribe(form: web::Form<User>, pool: web::Data<sqlx::PgPool>) -> HttpResponse {
  match insert_subscriber(&pool, &form).await {
    Ok(_) => HttpResponse::Ok().finish(),
    Err(_) => HttpResponse::InternalServerError().finish(),
  }
}

#[tracing::instrument(name = "Saving subscriber details in database", skip(pool, form))]
pub async fn insert_subscriber(pool: &sqlx::PgPool, form: &User) -> Result<(), sqlx::Error> {
  sqlx::query!(
    r#"
      INSERT INTO subscriptions (id, email, name, subscribed_at)
      VALUES ($1, $2, $3, $4)
    "#,
    Uuid::new_v4(),
    form.email,
    form.name,
    chrono::Local::now()
  )
  .execute(pool)
  .await
  .map_err(|error| {
    tracing::error!("Failed inserting subscriber: {error}");
    error
  })?;
  tracing::info!("Subscriber added successfully");
  Ok(())
}
