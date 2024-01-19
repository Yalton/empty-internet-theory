use redis::{Client, Commands};
use serde_json;

use crate::config::Config;
use crate::models::user::User;
use crate::models::post::Post;

pub struct RedisClient {
    client: Client,
}

impl RedisClient {
    pub fn new(config: &Config) -> Result<Self, redis::RedisError> {
        let client = Client::open(config.redis_url.as_str())?;
        Ok(RedisClient { client })
    }

    pub fn save_user(&self, user: &User) -> Result<(), redis::RedisError> {
        let mut conn = self.client.get_connection()?;
        let key = format!("user:{}", user.id);
        conn.set(key, serde_json::to_string(user)?)?;
        Ok(())
    }

    pub fn get_user(&self, user_id: u64) -> Result<Option<User>, redis::RedisError> {
        let mut conn = self.client.get_connection()?;
        let key = format!("user:{}", user_id);
        let user_json: Option<String> = conn.get(key)?;
        match user_json {
            Some(json) => Ok(Some(serde_json::from_str(&json)?)),
            None => Ok(None),
        }
    }

    pub fn save_post(&self, post: &Post) -> Result<(), redis::RedisError> {
        let mut conn = self.client.get_connection()?;
        let key = format!("post:{}", post.id);
        conn.set(key, serde_json::to_string(post)?)?;
        Ok(())
    }

    pub fn get_post(&self, post_id: u64) -> Result<Option<Post>, redis::RedisError> {
        let mut conn = self.client.get_connection()?;
        let key = format!("post:{}", post_id);
        let post_json: Option<String> = conn.get(key)?;
        match post_json {
            Some(json) => Ok(Some(serde_json::from_str(&json)?)),
            None => Ok(None),
        }
    }
}
