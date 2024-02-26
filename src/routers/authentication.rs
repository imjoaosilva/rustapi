use super::authentication_controller;

pub mod authentication {
    use axum::{routing::get, Router};
    use super::authentication_controller::*;
    
    pub fn router() -> Router {
        Router::new()
            .route("/login", get(login))
            .route("/register", get(register))
    }

}
