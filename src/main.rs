use axum::{
    routing::{get, post},
    Router, Json, extract::State,
};
use dotenvy::dotenv;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;

struct AppState {
    client: Client,
    supabase_url: String,
    supabase_key: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    let client = Client::new();
    let supabase_url = env::var("SUPABASE_URL").expect("SUPABASE_URL is not set");
    let supabase_key = env::var("SUPABASE_KEY").expect("SUPABASE_KEY is not set");
    
    // Create the shared state
    let state = Arc::new(AppState {
        client,
        supabase_url,
        supabase_key,
    });

    let app = Router::new()
        .route("/", get(root))
        .route("/add_user", post(add_user))
        .route("/users", get(list_users))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello Everyone!"
}

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    email: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ApiError {
    message: String,
}

// New endpoint to list all users
async fn list_users(
    State(state): State<Arc<AppState>>
) -> Result<Json<Vec<serde_json::Value>>, (axum::http::StatusCode, Json<ApiError>)> {
    let response = state.client
        .get(format!("{}/rest/v1/users", state.supabase_url))
        .header("apikey", &state.supabase_key)
        .header("Authorization", format!("Bearer {}", state.supabase_key))
        .send()
        .await
        .map_err(|e| (
            axum::http::StatusCode::BAD_GATEWAY, 
            Json(ApiError { message: format!("Failed to reach Supabase: {}", e) })
        ))?;

    if !response.status().is_success() {
        let status = response.status();
        let error_message = response.text().await
            .unwrap_or_else(|_| "Unknown error".to_string());
        
        return Err((
            axum::http::StatusCode::from_u16(status.as_u16()).unwrap_or(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
            Json(ApiError { message: error_message })
        ));
    }

    let users = response.json::<Vec<serde_json::Value>>().await
        .map_err(|e| (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR, 
            Json(ApiError { message: format!("Failed to parse response: {}", e) })
        ))?;
    
    Ok(Json(users))
}

// Modified add_user function to use the shared state
async fn add_user(
    State(state): State<Arc<AppState>>,
    Json(user): Json<User>
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, Json<ApiError>)> {
    let response = state.client
        .post(format!("{}/rest/v1/users", state.supabase_url))
        .header("apikey", &state.supabase_key)
        .header("Authorization", format!("Bearer {}", state.supabase_key))
        .header("Content-Type", "application/json")
        .header("Prefer", "return=representation")
        .json(&user)
        .send()
        .await
        .map_err(|e| (
            axum::http::StatusCode::BAD_GATEWAY, 
            Json(ApiError { message: format!("Failed to reach Supabase: {}", e) })
        ))?;

    if !response.status().is_success() {
        let status = response.status();
        let error_message = response.text().await
            .unwrap_or_else(|_| "Unknown error".to_string());
        
        return Err((
            axum::http::StatusCode::from_u16(status.as_u16()).unwrap_or(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
            Json(ApiError { message: error_message })
        ));
    }

    let json_response = response.json::<serde_json::Value>().await
        .map_err(|e| (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR, 
            Json(ApiError { message: format!("Failed to parse response: {}", e) })
        ))?;
    
    Ok(Json(json_response))
}