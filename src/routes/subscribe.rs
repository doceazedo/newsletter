use actix_web::{post, web};
use serde::Deserialize;

#[derive(Deserialize)]
struct SubscribeRequest {
    name: String,
    email: String,
}

#[post("/subscribe")]
async fn subscribe(req: web::Json<SubscribeRequest>) -> actix_web::Result<String> {
    Ok(format!("Hey {} ({})!", req.name, req.email))
}
