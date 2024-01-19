use axum::extract::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

pub mod status;

/// Common fallback method (404 Not Found).
pub async fn fallback() -> Error {
    Error::new("not found").status(StatusCode::NOT_FOUND)
}

/// The error type for [`axum::handler`] methods.
/// Constructable from [`anyhow::Error`] and convertable into [`Response`].
#[derive(Debug, Clone, Serialize)]
pub struct Error {
    reason: String,
    #[serde(skip_serializing)]
    status: StatusCode,
}

/// A specialized [`Result`] type for [`axum::handler`] methods.
pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    /// Creates the [`Error`] from the given reason.
    pub fn new(reason: impl AsRef<str>) -> Self {
        Self {
            reason: reason.as_ref().to_owned(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    /// Replaces the default [`StatusCode::INTERNAL_SERVER_ERROR`].
    pub fn status(mut self, status: StatusCode) -> Self {
        self.status = status;
        self
    }
}

impl<E: Into<anyhow::Error>> From<E> for Error {
    fn from(inner: E) -> Self {
        Self {
            reason: inner.into().to_string(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (self.status, Json(self)).into_response()
    }
}
