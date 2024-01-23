use axum::extract::{Json, State};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::handler::Result;
use crate::service::account::Account;

#[derive(Debug, Deserialize, TS)]
#[ts(export)]
pub struct SignUpRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub struct SignUpResponse {
    token: String,
}

#[tracing::instrument]
pub async fn sign_up(
    State(account): State<Account>,
    Json(body): Json<SignUpRequest>,
) -> Result<(StatusCode, Json<SignUpResponse>)> {
    // TODO: Custom error codes and reasons.
    let token = account.sign_in(&body.username, &body.password).await?;
    let response = SignUpResponse { token };
    Ok((StatusCode::CREATED, Json(response)))
}

#[derive(Debug, Deserialize, TS)]
#[ts(export)]
pub struct SignInRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub struct SignInResponse {
    token: String,
}

#[tracing::instrument]
pub async fn sign_in(
    State(account): State<Account>,
    Json(body): Json<SignInRequest>,
) -> Result<(StatusCode, Json<SignInResponse>)> {
    // TODO: Custom error codes and reasons.
    let token = account.sign_in(&body.username, &body.password).await?;
    let response = SignInResponse { token };
    Ok((StatusCode::OK, Json(response)))
}

#[derive(Debug, Deserialize, TS)]
#[ts(export)]
pub struct OneTimeRequest {
    username: String,
}

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub struct OneTimeResponse {
    token: String,
}

#[tracing::instrument]
pub async fn one_time(
    State(account): State<Account>,
    Json(body): Json<OneTimeRequest>,
) -> Result<(StatusCode, Json<OneTimeResponse>)> {
    // TODO: Custom error codes and reasons.
    let token = account.one_time(&body.username).await?;
    let response = OneTimeResponse { token };
    Ok((StatusCode::OK, Json(response)))
}
