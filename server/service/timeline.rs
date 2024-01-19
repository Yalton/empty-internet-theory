use std::convert::Infallible;

use crate::handler::status::CheckStatus;

#[derive(Debug, Clone)]
pub struct Timeline {}

impl Timeline {
    pub async fn connect() -> anyhow::Result<Self> {
        Ok(Self {})
    }
}

impl CheckStatus for Timeline {
    type Error = Infallible;
}
