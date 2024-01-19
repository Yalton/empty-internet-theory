use redis::{Client, Commands, RedisResult};
use crate::config::Config;
use crate::models::user::User;
use crate::models::post::Post;

mod redis_client;

pub struct RedisConnection {
    client: Client,
}

impl RedisConnection {
    pub fn new(config: &Config) -> RedisResult<Self> {
        let client = redis::Client::open(config.redis_url.as_str())?;
        Ok(RedisConnection { client })
    }

    pub fn get_client(&self) -> &Client {
        &self.client
    }
}

pub fn initialize_db(config: &Config) -> RedisResult<RedisConnection> {
    RedisConnection::new(config)
}

// You can add additional helper methods for interacting with the Redis database here.
// For example, methods to save and retrieve users and posts, etc.

// Example of a helper method to save a user to Redis
impl RedisConnection {
    pub fn save_user(&self, user: &User) -> RedisResult<()> {
        let mut conn = self.client.get_connection()?;
        let key = format!("user:{}", user.id);
        conn.set(key, serde_json::to_string(user)?)?;
        Ok(())
    }

    // Example of a helper method to retrieve a user from Redis
    pub fn get_user(&self, user_id: u64) -> RedisResult<Option<User>> {
        let mut conn = self.client.get_connection()?;
        let key = format!("user:{}", user_id);
        let user_json: Option<String> = conn.get(key)?;
        if let Some(json) = user_json {
            Ok(Some(serde_json::from_str(&json)?))
        } else {
            Ok(None)
        }
    }

    // Similar methods can be added for posts
    pub fn save_post(&self, post: &Post) -> RedisResult<()> {
        let mut conn = self.client.get_connection()?;
        let key = format!("post:{}", post.id);
        conn.set(key, serde_json::to_string(post)?)?;
        Ok(())
    }

    pub fn get_post(&self, post_id: u64) -> RedisResult<Option<Post>> {
        let mut conn = self.client.get_connection()?;
        let key = format!("post:{}", post_id);
        let post_json: Option<String> = conn.get(key)?;
        if let Some(json) = post_json {
            Ok(Some(serde_json::from_str(&json)?))
        } else {
            Ok(None)
        }
    }
}

// Re-export the Redis client module
pub use redis_client::RedisClient;
