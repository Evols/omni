use std::net::SocketAddr;

use axum::Router;

use crate::ports::Ports;

pub mod hello;
pub mod validate;

pub fn router() -> Router<Ports> {
    Router::new()
        .merge(hello::router())
        .merge(validate::router())
}

pub async fn run_server(ports: Ports) {
    let app = router().with_state(ports);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.expect("bind");

    axum::serve(listener, app).await.expect("serve");
}
