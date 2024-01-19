use warp::{http::StatusCode, reject, Rejection, Reply};
use serde_json::json;
use crate::db::redis_client::RedisClient;
use crate::models::User;
use crate::utils::error::ServiceError;

// Handler to create a new user
pub async fn create_user(user: User, redis_client: RedisClient) -> Result<impl Reply, Rejection> {
    match user.save(&redis_client) {
        Ok(_) => Ok(warp::reply::with_status(
            warp::reply::json(&json!({"message": "User created successfully"})),
            StatusCode::CREATED,
        )),
        Err(e) => Err(reject::custom(e)),
    }
}

// Handler to retrieve a user by ID
pub async fn get_user(user_id: String, redis_client: RedisClient) -> Result<impl Reply, Rejection> {
    match User::find_by_id(&user_id, &redis_client) {
        Ok(user) => Ok(warp::reply::json(&user)),
        Err(ServiceError::NotFound) => Ok(warp::reply::with_status(
            warp::reply::json(&json!({"error": "User not found"})),
            StatusCode::NOT_FOUND,
        )),
        Err(e) => Err(reject::custom(e)),
    }
}

// Handler to update a user
pub async fn update_user(user_id: String, update_data: User, redis_client: RedisClient) -> Result<impl Reply, Rejection> {
    match User::find_by_id(&user_id, &redis_client) {
        Ok(mut user) => {
            user.username = update_data.username;
            user.email = update_data.email;
            user.password = update_data.password; // Remember to hash the password in a real application
            match user.save(&redis_client) {
                Ok(_) => Ok(warp::reply::with_status(
                    warp::reply::json(&json!({"message": "User updated successfully"})),
                    StatusCode::OK,
                )),
                Err(e) => Err(reject::custom(e)),
            }
        }
        Err(ServiceError::NotFound) => Ok(warp::reply::with_status(
            warp::reply::json(&json!({"error": "User not found"})),
            StatusCode::NOT_FOUND,
        )),
        Err(e) => Err(reject::custom(e)),
    }
}

// Handler to delete a user
pub async fn delete_user(user_id: String, redis_client: RedisClient) -> Result<impl Reply, Rejection> {
    match User::delete_by_id(&user_id, &redis_client) {
        Ok(_) => Ok(warp::reply::with_status(
            warp::reply::json(&json!({"message": "User deleted successfully"})),
            StatusCode::OK,
        )),
        Err(ServiceError::NotFound) => Ok(warp::reply::with_status(
            warp::reply::json(&json!({"error": "User not found"})),
            StatusCode::NOT_FOUND,
        )),
        Err(e) => Err(reject::custom(e)),
    }
}

// Note: The User model should have a delete_by_id method implemented for the delete_user handler to work.
// This method should remove the user from the Redis database.
