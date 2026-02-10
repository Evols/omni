use std::net::SocketAddr;

use axum::{
    Router,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};

mod adapters;
mod ports;

use adapters::iced_notification::{IcedNotificationAdapter, channel};
use ports::notification::ValidationRequest;

fn main() -> iced::Result {
    let (adapter, daemon) = channel();

    let server_adapter = adapter.clone();
    std::thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("tokio runtime");

        runtime.block_on(run_server(server_adapter));
    });

    daemon.run()
}

async fn run_server(adapter: IcedNotificationAdapter) {
    let app = Router::new()
        .route("/validate", post(validate))
        .route("/hello", get(hello))
        .with_state(adapter);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.expect("bind");

    axum::serve(listener, app).await.expect("serve");
}

async fn validate(State(adapter): State<IcedNotificationAdapter>) -> impl IntoResponse {
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

async fn hello() -> impl IntoResponse {
    (StatusCode::OK, "hello world!")
}
