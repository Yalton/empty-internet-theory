use axum::extract::{Json, State};
use axum::http::StatusCode;
use serde::Serialize;

use crate::handler::Result;
use crate::service::StatusService;

#[axum::async_trait]
pub trait CheckStatus {
    /// Used as a reason of the bad status check.
    /// TODO: Infallible as a default error type.
    type Error: std::error::Error;

    /// Returns `Ok()` if the service is healthy, reason otherwise.
    async fn check(&self) -> std::result::Result<(), Self::Error> {
        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct StatusRequest {}

#[derive(Debug, Serialize)]
pub struct StatusResponse {}

pub fn status(
    State(status): State<StatusService>,
    Json(body): Json<StatusRequest>,
) -> Result<(StatusCode, StatusResponse)> {
    let response = StatusResponse {};
    Ok((StatusCode::OK, response))
}
