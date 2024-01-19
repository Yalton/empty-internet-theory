use axum::http::StatusCode;
use tower::{Layer, Service};

use crate::handler::Error;

/// Transforms any known [`tower::BoxError`] into the [`Error`].
pub async fn handle_box_error(err: tower::BoxError) -> Error {
    if err.is::<tower::timeout::error::Elapsed>() {
        Error::new("request took too long").status(StatusCode::REQUEST_TIMEOUT)
    } else {
        Error::new("unhandled internal error")
    }
}

// TODO: Use tower::util::BoxCloneServiceLayer?
pub async fn instantiate_stack() -> anyhow::Result<()> {
    Ok(())
}
