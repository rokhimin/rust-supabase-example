use axum::{
    routing::{get, post},
    Router, Json,
};
use dotenvy::dotenv;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::net::SocketAddr;
use hyper::Server; // Gunakan ini
use tokio;

#[tokio::main]
async fn main() {
    // Load .env
    dotenv().ok();

    // Router dengan endpoint
    let app = Router::new()
        .route("/", get(root))
        .route("/add_user", post(add_user));

    // Jalankan server di port 3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello from Axum!"
}

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    email: String,
}

async fn add_user(Json(user): Json<User>) -> Json<serde_json::Value> {
    let supabase_url = env::var("SUPABASE_URL").expect("SUPABASE_URL is not set");
    let supabase_key = env::var("SUPABASE_KEY").expect("SUPABASE_KEY is not set");

    let client = Client::new();
    let response = client
        .post(format!("{}/rest/v1/users", supabase_url))
        .header("apikey", &supabase_key)
        .header("Authorization", format!("Bearer {}", supabase_key))
        .header("Content-Type", "application/json")
        .header("Prefer", "return=representation")
        .json(&user)
        .send()
        .await
        .unwrap();

    let json_response = response.json::<serde_json::Value>().await.unwrap();
    Json(json_response)
}
