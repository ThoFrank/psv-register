use axum::{http::StatusCode, response::IntoResponse, Json};
use common::archer::Archer;

pub async fn create_archer(Json(payload): Json<Archer>) -> impl IntoResponse {
    (StatusCode::CREATED, Json(payload))
}

pub async fn list_archers() -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, "501 Not implemented!")
}