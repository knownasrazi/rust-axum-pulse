use axum::{routing::get, Router};

async fn root() -> String {
    "rust-axum-pulse - Pulse, in Rust.".to_string()
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
