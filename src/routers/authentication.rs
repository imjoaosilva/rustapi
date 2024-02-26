

pub mod authentication {
    use std::sync::Arc;

    use axum::{routing::get, Extension, Router};

    use crate::models::state::State;

    pub fn router() -> Router {
        Router::new()
            .route("/login", get(login))
            .route("/register", get(register))
    }

    // login route que pega a layer com a extension do prisma client

    async fn login(Extension(_client): Extension<Arc<State>>) -> &'static str {
        "Login"
    }

    async fn register() -> &'static str {
        "Register"
    }
}
