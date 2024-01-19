use std::convert::Infallible;

use crate::handler::status::CheckStatus;

#[derive(Debug, Clone)]
pub struct StatusService {}

impl StatusService {
    pub async fn connect() -> anyhow::Result<Self> {
        Ok(Self {})
    }
}

impl CheckStatus for StatusService {
    type Error = Infallible;
}
