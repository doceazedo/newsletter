use actix_web::{HttpResponse, post, web};
use serde::Deserialize;
use sqlx::PgPool;
use sqlx::types::chrono::Utc;
use sqlx::types::Uuid;

#[derive(Deserialize)]
struct SubscribeRequest {
    name: String,
    email: String,
}

#[tracing::instrument(
    name = "Creating new subscription",
    skip(req, db),
    fields(
        email = req.email,
        name = req.name
    )
)]
#[post("/subscribe")]
async fn subscribe(req: web::Json<SubscribeRequest>, db: web::Data<PgPool>) -> HttpResponse {
    match insert_subscription(&req, &db).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(
    name = "Inserting data",
    skip(req, db),
)]
async fn insert_subscription(req: &SubscribeRequest, db: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        req.email,
        req.name,
        Utc::now()
    )
        .execute(db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to insert data: {:?}", e);
            e
        })?;
    Ok(())
}