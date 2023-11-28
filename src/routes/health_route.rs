use actix_web::{get, Responder, HttpResponse};
use serde_json::json;
#[get("/api/health_checker")]
pub async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Build simle crud";
    HttpResponse::Ok().json(json!({"statuc": "success", "message": MESSAGE}))
}
