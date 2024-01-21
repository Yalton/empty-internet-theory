use std::convert::Infallible;

use crate::handler::status::CheckStatus;

#[derive(Debug, Clone)]
pub struct Account {}

impl Account {
    pub async fn connect() -> anyhow::Result<Self> {
        Ok(Self {})
    }
}

impl CheckStatus for Account {
    type Error = Infallible;
}
