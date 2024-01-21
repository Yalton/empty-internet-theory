use axum::extract::{Json, State};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::handler::Result;
use crate::service::account::Account;

#[derive(Debug, Deserialize, TS)]
#[ts(export)]
pub struct SignUpRequest {}

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub struct SignUpResponse {}

#[tracing::instrument]
pub async fn sign_up(
    State(account): State<Account>,
    Json(body): Json<SignUpRequest>,
) -> Result<(StatusCode, Json<SignUpResponse>)> {
    // TODO.
    let response = SignUpResponse {};
    Ok((StatusCode::OK, Json(response)))
}

#[derive(Debug, Deserialize, TS)]
#[ts(export)]
pub struct SignInRequest {}

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub struct SignInResponse {}

#[tracing::instrument]
pub async fn sign_in(
    State(account): State<Account>,
    Json(body): Json<SignInRequest>,
) -> Result<(StatusCode, Json<SignInResponse>)> {
    // TODO.
    let response = SignInResponse {};
    Ok((StatusCode::OK, Json(response)))
}

#[derive(Debug, Deserialize, TS)]
#[ts(export)]
pub struct VisitRequest {}

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub struct VisitResponse {}

#[tracing::instrument]
pub async fn visit(
    State(account): State<Account>,
    Json(body): Json<VisitRequest>,
) -> Result<(StatusCode, Json<VisitResponse>)> {
    // TODO.
    let response = VisitResponse {};
    Ok((StatusCode::OK, Json(response)))
}
