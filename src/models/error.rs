use axum::{body::Body, http, response::IntoResponse};
use prisma_client_rust::QueryError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Query error")]
    Query(#[from] QueryError),

    #[error("Bad request")]
    BadRequest(),
}

impl IntoResponse for Error {
    fn into_response(self) -> http::Response<Body> {
        match self {
            Error::Query(_) => http::Response::builder()
                .status(500)
                .body(Body::from("Internal server error"))
                .unwrap(),
            Error::BadRequest() => {
                let error = r#"{ "status": 400, "message": "Bad request" }"#;

                http::Response::builder()
                    .status(400)
                    .body(Body::from(error))
                    .unwrap()
            }
        }
    }
}