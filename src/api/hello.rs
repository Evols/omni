use axum::{Router, http::StatusCode, response::IntoResponse, routing::get};

use crate::ports::Ports;

pub fn router() -> Router<Ports> {
    Router::new().route("/hello", get(handle))
}

async fn handle() -> impl IntoResponse {
    (StatusCode::OK, "hello world!")
}
