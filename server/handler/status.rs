use axum::extract::{Json, State};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::handler::Result;
use crate::service::account::Account;
use crate::service::timeline::Timeline;

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

#[derive(Debug, Deserialize, TS)]
#[ts(export)]
pub struct StatusRequest {
    verbose: bool,
}

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub struct StatusResponse {
    healthy: bool,
}

#[tracing::instrument]
pub async fn check(
    State(account): State<Account>,
    State(timeline): State<Timeline>,
    Json(body): Json<StatusRequest>,
) -> Result<(StatusCode, Json<StatusResponse>)> {
    // TODO.
    let response = StatusResponse { healthy: true };
    Ok((StatusCode::OK, Json(response)))
}
