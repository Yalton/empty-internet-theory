use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;
use redis::RedisError;
use serde_json::error::Error as SerdeJsonError;
use std::fmt;

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),

    #[display(fmt = "Unauthorized")]
    Unauthorized,

    #[display(fmt = "RedisError: {}", _0)]
    RedisError(String),

    #[display(fmt = "SerdeJsonError: {}", _0)]
    SerdeJsonError(String),
}

// Implement ResponseError trait for ServiceError to convert ServiceErrors to HTTP responses
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error")
            }
            ServiceError::BadRequest(ref message) => {
                HttpResponse::BadRequest().json(message)
            }
            ServiceError::Unauthorized => {
                HttpResponse::Unauthorized().json("Unauthorized")
            }
            ServiceError::RedisError(ref message) => {
                HttpResponse::InternalServerError().json(message)
            }
            ServiceError::SerdeJsonError(ref message) => {
                HttpResponse::InternalServerError().json(message)
            }
        }
    }
}

// Implement From trait for RedisError to convert RedisError to ServiceError
impl From<RedisError> for ServiceError {
    fn from(error: RedisError) -> ServiceError {
        ServiceError::RedisError(error.to_string())
    }
}

// Implement From trait for SerdeJsonError to convert SerdeJsonError to ServiceError
impl From<SerdeJsonError> for ServiceError {
    fn from(error: SerdeJsonError) -> ServiceError {
        ServiceError::SerdeJsonError(error.to_string())
    }
}

// Implement fmt::Display for ServiceError to enable printing to the console
impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
