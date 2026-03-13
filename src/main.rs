use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    // Initialize logging so we can see what's happening
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/health", get(health_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    tracing::info!("Server running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}

async fn health_handler() -> &'static str {
    "ok"
}
