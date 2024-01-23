use serde::Serialize;

use crate::service::account::Account;
use crate::service::leaders::Leaderboard;
use crate::service::timeline::Timeline;

pub mod account;
pub mod leaders;
pub mod timeline;

#[derive(Debug, Clone)]
pub struct State {
    account: Account,
    leaderboard: Leaderboard,
    timeline: Timeline,
}

impl State {
    pub async fn connect() -> anyhow::Result<Self> {
        Ok(Self {
            account: Account::connect().await?,
            leaderboard: Leaderboard::new(),
            timeline: Timeline::new(),
        })
    }
}

macro_rules! impl_di {
    ($($f:ident: $t:ty),+) => {$(
        impl axum::extract::FromRef<State> for $t {
            fn from_ref(state: &State) -> Self {
                state.$f.clone()
            }
        }
    )+};
}

impl_di!(account: Account);
impl_di!(leaderboard: Leaderboard);
impl_di!(timeline: Timeline);
