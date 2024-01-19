use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug)]
pub struct Content {
    pub text_content: String,
    pub image: String
}

impl Content {
    pub fn new(text_content: u64, image: String) -> Content {
        Content {
            text_content,
            image
        }
    }
}
