use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::content; 

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    pub id: u64,
    pub user_id: u64,
    pub content: Content,
    pub timestamp: u64,
}

impl Post {
    pub fn new(id: u64, user_id: u64, content: String) -> Post {
        Post {
            id,
            user_id,
            content,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
        }
    }
}
