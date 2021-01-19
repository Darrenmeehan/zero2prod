use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(
    form: web::Form<FormData>,
    // Retrieving a connection from the application state!
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
    let request_id = Uuid::new_v4();
    log::info!(
        "request_id {} - Adding `{}` `{}` as a new subscriber.",
        request_id,
        form.email,
        form.name
    );
    log::info!(
        "request_id {} - Saving new subscriber details in the database",
        request_id
    );
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await
    .map_err(|e| {
        log::error!("Failed to execute query: {:?}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    log::info!(,
        "request_id {} - New subscriber details have been saved",
        request_id
    );
    Ok(HttpResponse::Ok().finish())
}
