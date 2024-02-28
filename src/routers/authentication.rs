
use axum::{routing::get,routing::post, Router};
use super::{login,register};


/// Creates and returns a router for authentication endpoints.
///
/// # Examples
///
/// ```
/// use rustapi::routers::authentication::router;
///
/// let auth_router = router();
/// // Use the `auth_router` to handle authentication routes
/// ```

pub fn router() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/register", get(register))
}
