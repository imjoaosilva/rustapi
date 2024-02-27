use super::authentication_controller;

pub mod authentication {
    use axum::{routing::get,routing::post, Router};
    use super::authentication_controller::*;
    
    pub fn router() -> Router {
        Router::new()
            .route("/login", post(login))
            .route("/register", get(register))
    }

}
