use actix_web::{web, HttpResponse};
use uuid::Uuid;
use tracing::Instrument;

#[allow(unused)]
#[derive(serde::Deserialize)]
pub struct User {
  name: String,
  email: String,
}

pub async fn subscribe(form: web::Form<User>, pool: web::Data<sqlx::PgPool>) -> HttpResponse {
  let request_id = uuid::Uuid::new_v4().to_string();

  let request_span = tracing::info_span!(
    "Adding new subscriber",
    %request_id,
    sub_name = %form.name,
    sub_email = %form.email
  );

  let _req_span_guard = request_span.enter();

  let query_span = tracing::info_span!("Saving new subscriber details into the database");
  match sqlx::query!(
    r#"
      INSERT INTO subscriptions (id, email, name, subscribed_at)
      VALUES ($1, $2, $3, $4)
    "#,
    Uuid::new_v4(),
    form.email,
    form.name,
    chrono::Local::now()
  )
  .execute(pool.get_ref())
  .instrument(query_span)
  .await
  {
    Ok(_) => {
      tracing::info!("(req_id: {request_id}) New subscriber added successfully");
      HttpResponse::Ok().finish()
    },
    Err(error) => {
      tracing::error!("(req_id: {request_id}) Error when adding subscriber: {:?}", error);
      HttpResponse::InternalServerError().body("Error creating subscription")
    }
  }
}
