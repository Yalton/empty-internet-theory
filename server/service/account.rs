use std::convert::Infallible;

use crate::handler::status::CheckStatus;
use crate::internal::types::{RoomId, Token, UserId};

#[derive(Debug, Clone)]
pub struct Account {}

impl Account {
    pub async fn connect() -> anyhow::Result<Self> {
        Ok(Self {})
    }

    pub async fn sign_up(&self, username: &str, password: &str) -> anyhow::Result<Token> {
        todo!()
    }

    pub async fn sign_in(&self, username: &str, password: &str) -> anyhow::Result<Token> {
        todo!()
    }

    pub async fn one_time(&self, username: &str) -> anyhow::Result<Token> {
        todo!()
    }

    pub async fn auth(&self, token: &Token) -> anyhow::Result<(UserId, Option<RoomId>)> {
        todo!()
    }
}

impl CheckStatus for Account {
    type Error = Infallible;
}
