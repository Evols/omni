use axum::{Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};

use crate::ports::notification::ValidationRequest;
use crate::ports::Ports;

pub fn router() -> Router<Ports> {
    Router::new().route("/validate", post(handle))
}

async fn handle(State(ports): State<Ports>) -> impl IntoResponse {
    let request = ValidationRequest {
        title: "Validation required".to_string(),
        message: "Please approve this action.".to_string(),
    };

    let approved = ports.notification_port.request_validation(request).await;

    if approved {
        (StatusCode::OK, "approved")
    } else {
        (StatusCode::FORBIDDEN, "rejected")
    }
}
