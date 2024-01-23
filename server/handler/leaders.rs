use std::num::NonZeroU32;

use axum::extract::{Json, State};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::handler::Result;
use crate::service::leaders::{LeaderPosition, Leaderboard};

#[derive(Debug, Deserialize, TS)]
#[ts(export)]
pub struct LeaderboardRequest {
    positions: u32,
}

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub struct LeaderboardResponse {
    positions: Vec<LeaderPosition>,
}

#[tracing::instrument]
pub async fn top(
    State(leaderboard): State<Leaderboard>,
    body: Option<Json<LeaderboardRequest>>,
) -> Result<(StatusCode, Json<LeaderboardResponse>)> {
    // TODO: Move to consts.
    let positions_by_default: NonZeroU32 = NonZeroU32::new(20).expect("should be positive");

    let positions = body
        .and_then(|x| NonZeroU32::new(x.positions))
        .unwrap_or(positions_by_default);

    let response = LeaderboardResponse {
        positions: leaderboard.current(positions).await?,
    };

    Ok((StatusCode::OK, Json(response)))
}
