use axum::{Extension, Router};
use std::{env, sync::Arc};
use rustapi::PrismaClient;
use rustapi::models::state::State;

pub mod models;


#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let client = Arc::new(State {
        client: PrismaClient::_builder()
            .build()
            .await
            .expect("Failed to connect to database")
    });

    let port = env::var("PORT").unwrap_or("3035".to_string());

    let app = Router::new()
        .nest("/api/v1/auth", rustapi::routers::authentication())
        .layer(Extension(client));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    println!("🚀 Server running on port {}", port);

    axum::serve(listener, app).await.unwrap();
}
