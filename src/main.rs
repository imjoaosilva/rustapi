use std::env;
use axum::Router;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let port = env::var("PORT").unwrap_or("8080".to_string());

    let app = Router::new()
        .nest("/api/v1/auth", rustapi::routers::authentication());

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();

    println!("🚀 Server running on port {}", port);

    axum::serve(listener, app).await.unwrap();
}
