
use axum::{routing::get,routing::post, Router};
use super::{login,register};
    
pub fn router() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/register", get(register))
}
