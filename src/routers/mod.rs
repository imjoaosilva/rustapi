mod authentication;

pub use super::controllers::authentication::{login, register};

pub use authentication::router as authentication;
