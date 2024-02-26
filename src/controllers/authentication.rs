pub mod authentication_controller {

    use std::sync::Arc;

    use axum::Extension;

    use crate::models::state::State;

    pub async fn login(Extension(_client): Extension<Arc<State>>) -> &'static str {
        "Login"
    }

    pub async fn register() -> &'static str {
        "Register"
    }
}