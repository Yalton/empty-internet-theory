pub mod user_handlers;
pub mod post_handlers;

use crate::db::redis_client::RedisClient;
use crate::models::{User, Post};
use crate::utils::error::ServiceError;
use warp::http::StatusCode;

pub async fn handle_create_user(
    new_user: User,
    redis_client: RedisClient,
) -> Result<impl warp::Reply, warp::Rejection> {
    new_user
        .save(&redis_client)
        .map_err(|e| warp::reject::custom(e))?;

    Ok(warp::reply::with_status(
        "User created successfully",
        StatusCode::CREATED,
    ))
}

pub async fn handle_get_user(
    user_id: String,
    redis_client: RedisClient,
) -> Result<impl warp::Reply, warp::Rejection> {
    match User::find_by_id(&user_id, &redis_client) {
        Ok(user) => Ok(warp::reply::json(&user)),
        Err(e) => match e {
            ServiceError::NotFound => Err(warp::reject::not_found()),
            _ => Err(warp::reject::custom(e)),
        },
    }
}

pub async fn handle_create_post(
    new_post: Post,
    redis_client: RedisClient,
) -> Result<impl warp::Reply, warp::Rejection> {
    new_post
        .save(&redis_client)
        .map_err(|e| warp::reject::custom(e))?;

    Ok(warp::reply::with_status(
        "Post created successfully",
        StatusCode::CREATED,
    ))
}

pub async fn handle_get_post(
    post_id: u64,
    redis_client: RedisClient,
) -> Result<impl warp::Reply, warp::Rejection> {
    match Post::find_by_id(post_id, &redis_client) {
        Ok(post) => Ok(warp::reply::json(&post)),
        Err(e) => match e {
            ServiceError::NotFound => Err(warp::reject::not_found()),
            _ => Err(warp::reject::custom(e)),
        },
    }
}

// Here you can add more handler functions for different routes
