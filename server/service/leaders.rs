use std::num::NonZeroU32;

use serde::Serialize;
use ts_rs::TS;

#[derive(Debug, Clone)]
pub struct Leaderboard {}

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub struct LeaderPosition {
    username: String,
    with_tweets: u32,
    with_likes: u32,
    with_reports: u32,
}

impl Leaderboard {
    pub fn new() -> Self {
        // TODO: Connect?
        Self {}
    }

    #[tracing::instrument]
    pub async fn current(&self, positions: NonZeroU32) -> anyhow::Result<Vec<LeaderPosition>> {
        // TODO: Load?
        let leaders = (1..=positions.get())
            .map(|x| LeaderPosition {
                username: "John Debugino".to_string(),
                with_tweets: x,
                with_likes: x,
                with_reports: x,
            })
            .rev()
            .collect();

        Ok(leaders)
    }
}
