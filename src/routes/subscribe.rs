use actix_web::{post, web, HttpResponse};
use serde::Deserialize;
use sqlx::types::chrono::Utc;
use sqlx::types::Uuid;
use sqlx::PgPool;

#[derive(Deserialize)]
struct SubscribeRequest {
    name: String,
    email: String,
}

#[post("/subscribe")]
async fn subscribe(req: web::Json<SubscribeRequest>, db: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        req.email,
        req.name,
        Utc::now()
    )
    .execute(db.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to insert subscription: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
