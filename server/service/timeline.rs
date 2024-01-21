use std::collections::HashMap;
use std::convert::Infallible;

use serde::Deserialize;

use crate::handler::status::CheckStatus;

pub type RoomId = String;

#[derive(Debug, Deserialize)]
pub struct CreateRoom {}

#[derive(Debug, Deserialize)]
pub struct JoinRoom {}

#[derive(Debug, Deserialize)]
pub struct Tweet {}

#[derive(Debug, Deserialize)]
pub struct Retweet {}

#[derive(Debug, Default, Clone)]
pub struct Timeline {
    rooms: HashMap<String, ()>,
}

impl Timeline {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn connect() -> anyhow::Result<Self> {
        Ok(Self {
            rooms: HashMap::default(),
        })
    }

    pub async fn create_tweet(&self, user: &str, message: &str) -> anyhow::Result<()> {
        // TODO: Only letters,numbers and -+&^*/,.:;

        Ok(())
    }

    // pub async fn create_retweet(&self, )

    // pub fn create
}

impl CheckStatus for Timeline {
    type Error = Infallible;
}
