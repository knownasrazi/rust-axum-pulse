use axum::{routing::{get, post}, Json, Router, extract::Path, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, Clone)]
struct Item { id: u32, name: String, price: f64 }

type Store = Arc<Mutex<Vec<Item>>>;

async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({"status":"ok"}))
}

async fn list_items(store: axum::extract::State<Store>) -> Json<Vec<Item>> {
    let items = store.lock().unwrap().clone();
    Json(items)
}

async fn create_item(store: axum::extract::State<Store>, Json(payload): Json<Item>) -> (StatusCode, Json<Item>) {
    let mut items = store.lock().unwrap();
    let mut new_item = payload;
    new_item.id = (items.len() as u32) + 1;
    items.push(new_item.clone());
    (StatusCode::OK, Json(new_item))
}

async fn get_item(Path(id): Path<u32>, store: axum::extract::State<Store>) -> Result<Json<Item>, StatusCode> {
    let items = store.lock().unwrap();
    for item in items.iter() {
        if item.id == id {
            return Ok(Json(item.clone()));
        }
    }
    Err(StatusCode::NOT_FOUND)
}

#[tokio::main]
async fn main() {
    let store: Store = Arc::new(Mutex::new(vec![]));
    let app = Router::new()
        .route("/", get(health))
        .route("/health", get(health))
        .route("/items", get(list_items).post(create_item))
        .route("/items/:id", get(get_item))
        .with_state(store);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("rust-axum-pulse listening on :3000 — hand-crafted");
    axum::serve(listener, app).await.unwrap();
}
