use axum::{Router, http::StatusCode, response::IntoResponse, routing::get};

use crate::adapters::iced_notification::IcedNotificationAdapter;

pub fn router() -> Router<IcedNotificationAdapter> {
    Router::new().route("/hello", get(handle))
}

async fn handle() -> impl IntoResponse {
    (StatusCode::OK, "hello world!")
}
