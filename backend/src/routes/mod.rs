use warp::Filter;

use crate::handlers::{user_handlers, post_handlers};
use crate::db::redis_client::RedisClient;
use crate::websockets::socket;

pub fn api_routes(redis_client: RedisClient) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    user_routes(redis_client.clone())
        .or(post_routes(redis_client.clone()))
        .or(ws_routes(redis_client))
}

/// User-related routes
fn user_routes(redis_client: RedisClient) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("users")
        .and(
            warp::post()
                .and(warp::body::json())
                .and(with_redis_client(redis_client.clone()))
                .and_then(user_handlers::create_user)
        )
        .or(
            warp::get()
                .and(warp::path::param())
                .and(with_redis_client(redis_client.clone()))
                .and_then(user_handlers::get_user)
        )
}

/// Post-related routes
fn post_routes(redis_client: RedisClient) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("posts")
        .and(
            warp::post()
                .and(warp::body::json())
                .and(with_redis_client(redis_client.clone()))
                .and_then(post_handlers::create_post)
        )
        .or(
            warp::get()
                .and(warp::path::param())
                .and(with_redis_client(redis_client.clone()))
                .and_then(post_handlers::get_post)
        )
}

/// WebSocket routes
fn ws_routes(redis_client: RedisClient) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("ws")
        .and(warp::ws())
        .and(with_redis_client(redis_client))
        .and_then(socket::ws_handler)
}

/// This function moves the RedisClient into the closure, allowing it to be used by the handlers.
fn with_redis_client(
    redis_client: RedisClient,
) -> impl Filter<Extract = (RedisClient,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || redis_client.clone())
}
