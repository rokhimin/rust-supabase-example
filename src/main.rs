use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let supabase_url = "https://psvmzuzlkfoippligqju.supabase.co";
    let supabase_key = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6InBzdm16dXpsa2ZvaXBwbGlncWp1Iiwicm9sZSI6ImFub24iLCJpYXQiOjE3NDIyMTQ2NjAsImV4cCI6MjA1Nzc5MDY2MH0.uQPuWKuZVwArLXMGRTrfxTMv05pgGfWcqnhmjNBnnxU";

    let client = Client::new();
    
    // Contoh: Insert data ke Supabase
    let new_user = User {
        name: "Richard".to_string(),
        email: "john@example.com".to_string(),
    };

    let response = client
        .post(format!("{}/rest/v1/users", supabase_url))
        .header("apikey", supabase_key)
        .header("Authorization", format!("Bearer {}", supabase_key))
        .header("Content-Type", "application/json")
        .header("Prefer", "return=representation")
        .json(&new_user)
        .send()
        .await?;

        let status = response.status();
        let body = response.text().await?;
        
        println!("HTTP Status: {}", status);
        println!("Response Body: {}", body);

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    email: String,
}