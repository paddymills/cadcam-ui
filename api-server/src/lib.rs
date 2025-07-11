use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::sync::Arc;
use thiserror::Error;

mod config;
pub use config::Config;

pub mod db;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("Failed to parse config")]
    ConfigParseError(#[from] toml::de::Error),
    #[error("I/O error")]
    IoError(#[from] std::io::Error),
    #[error("Database error")]
    DatabaseError(#[from] tiberius::error::Error),
    #[error("Database pool error")]
    DatabasePoolConnectorError(#[from] bb8_tiberius::Error),
    #[error("Database connection error")]
    DatabasePoolError(#[from] bb8::RunError<bb8_tiberius::Error>),
    #[error("Logging initialization error")]
    LoggingInitError(#[from] fern::InitError),
    #[error("Logging error")]
    LoggingError(#[from] log::SetLoggerError),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        log::error!("Server error: {}", self);
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}

#[derive(Debug)]
pub struct AppState {
    pub pool: db::DbPool,
}

pub type Result<T> = std::result::Result<T, ServerError>;
pub type ApiState = State<Arc<AppState>>;
