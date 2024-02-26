use super::controllers::authentication::authentication_controller;

mod authentication;

pub use authentication::authentication::router as authentication;