use axum::{
    Router,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::post,
};

use crate::adapters::iced_notification::IcedNotificationAdapter;
use crate::ports::notification::ValidationRequest;

pub fn router() -> Router<IcedNotificationAdapter> {
    Router::new().route("/validate", post(handle))
}

async fn handle(State(adapter): State<IcedNotificationAdapter>) -> impl IntoResponse {
    let request = ValidationRequest {
        title: "Validation required".to_string(),
        message: "Please approve this action.".to_string(),
    };

    let approved = adapter.request_validation(request).await;

    if approved {
        (StatusCode::OK, "approved")
    } else {
        (StatusCode::FORBIDDEN, "rejected")
    }
}
