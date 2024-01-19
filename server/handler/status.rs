use axum::extract::{Json, State};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::handler::Result;
use crate::service::Timeline;

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

#[derive(Debug, Deserialize)]
pub struct StatusRequest {}

#[derive(Debug, Serialize)]
pub struct StatusResponse {}

#[tracing::instrument]
pub async fn check(
    State(status): State<Timeline>,
    Json(body): Json<StatusRequest>,
) -> Result<(StatusCode, Json<StatusResponse>)> {
    let response = StatusResponse {};
    Ok((StatusCode::OK, Json(response)))
}
