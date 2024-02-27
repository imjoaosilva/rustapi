
use axum::{body::Body, extract::rejection::JsonRejection, http, Extension, Json};
use std::sync::Arc;
use crate::models::{error::Error, state::State, response::{JsonResponse, Login}};

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