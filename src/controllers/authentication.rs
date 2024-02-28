
use axum::{body::Body, extract::rejection::JsonRejection, http, Extension, Json};
use std::sync::Arc;
use crate::models::{error::Error, state::State, response::{JsonResponse, Login}};

/// Attempts to log in a user.
///
/// This function processes a login request by validating the provided JSON payload.
/// It expects a JSON object that matches the `Login` structure.
///
/// # Arguments
/// * `client` - An `Extension` wrapping an `Arc<State>`, representing the shared application state.
/// * `payload` - A JSON payload representing the login attempt. This can either be a valid `Json<Login>` object
///               or a `JsonRejection` if the input does not match the expected format.
///
/// # Returns
/// On success, returns an `http::Response<Body>` containing a JSON representation of the login result.
/// On failure, returns an `Error` indicating what went wrong, such as `Error::BadRequest()` if the payload is incorrect.
///

pub async fn login(
    client: Extension<Arc<State>>,
    payload: Result<Json<Login>, JsonRejection>
) -> Result<http::Response<Body>, Error> {

    if payload.is_err() {
        return Err(Error::BadRequest());
    };

    let result = client.client.user().find_many(vec![]).exec().await?;

    let json = serde_json::to_string(&JsonResponse {
        status: 200,
        data: result
    }).unwrap();
    
    let response = http::Response::builder()
        .status(200)
        .body(Body::from(json))
        .unwrap();

    Ok(response)
}

pub async fn register() -> &'static str {
    "Register"
}