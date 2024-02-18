pub mod authentication {
    use axum::{Router, routing::get};

    pub fn router() -> Router {
        Router::new()
            .route("/login", get(login))
            .route("/register", get(register))
    }

    async fn login() -> &'static str {
        "Login"
    }

    async fn register() -> &'static str {
        "Register"
    }
}
