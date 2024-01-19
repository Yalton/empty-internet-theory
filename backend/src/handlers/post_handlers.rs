use warp::{http::StatusCode, reject, reply::json, Reply};
use serde_json::json;

use crate::db::redis_client::RedisClient;
use crate::models::Post;
use crate::utils::error::ServiceError;

pub async fn create_post_handler(post: Post, redis_client: RedisClient) -> Result<impl Reply, reject::Rejection> {
    match redis_client.save_post(&post) {
        Ok(_) => Ok(json(&json!({"status": "success", "message": "Post created successfully."}))),
        Err(_) => Err(reject::custom(ServiceError::RedisError)),
    }
}

pub async fn get_post_handler(post_id: u64, redis_client: RedisClient) -> Result<impl Reply, reject::Rejection> {
    match redis_client.get_post(post_id) {
        Ok(Some(post)) => Ok(json(&post)),
        Ok(None) => Ok(json(&json!({"status": "error", "message": "Post not found."}))),
        Err(_) => Err(reject::custom(ServiceError::RedisError)),
    }
}

pub async fn delete_post_handler(post_id: u64, redis_client: RedisClient) -> Result<impl Reply, reject::Rejection> {
    match redis_client.delete_post(post_id) {
        Ok(_) => Ok(json(&json!({"status": "success", "message": "Post deleted successfully."}))),
        Err(_) => Err(reject::custom(ServiceError::RedisError)),
    }
}

// You may need to implement the delete_post method in the RedisClient if it does not exist.
// Here is a possible implementation of the delete_post method in the RedisClient struct:

// pub fn delete_post(&self, post_id: u64) -> Result<(), redis::RedisError> {
//     let mut conn = self.client.get_connection()?;
//     let key = format!("post:{}", post_id);
//     conn.del(key)?;
//     Ok(())
// }
