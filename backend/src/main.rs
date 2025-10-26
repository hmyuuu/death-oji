mod gugugaga;
mod handlers;
mod models;

use axum::{
    routing::{get, post, put},
    Router,
};
use handlers::AppState;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState {
        gugugaga_client: gugugaga::gugugagaClient::new(),
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(handlers::health_check))
        .route("/api/stream/start", post(handlers::start_stream))
        .route("/api/stream/stop", post(handlers::stop_stream))
        .route("/api/stream/update", put(handlers::update_stream))
        .route("/api/user/info", post(handlers::get_user_info))
        .route("/api/partitions", get(handlers::get_partitions))
        .route("/api/qrcode/generate", get(handlers::generate_qrcode))
        .route("/api/qrcode/poll", post(handlers::poll_qrcode))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:11451")
        .await
        .unwrap();

    println!("Server running on http://0.0.0.0:11451");

    axum::serve(listener, app).await.unwrap();
}
