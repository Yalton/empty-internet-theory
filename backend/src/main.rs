use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;

mod config;
mod db;
mod grpc;
mod handlers;
mod models;
mod routes;
mod utils;
mod websockets;

#[tokio::main]
async fn main() {
    // Load configuration
    let config = config::Config::new().expect("Failed to load configuration.");

    // Initialize Redis client
    let redis_client = db::redis_client::init_redis_client(&config.redis_url)
        .await
        .expect("Failed to create Redis client.");

    // Wrap Redis client in Arc and Mutex for shared state across async tasks
    let shared_redis_client = Arc::new(Mutex::new(redis_client));

    // Initialize gRPC server
    let grpc_server = grpc::service::serve(shared_redis_client.clone());

    // Initialize WebSocket server
    let websocket_server = websockets::socket::serve(shared_redis_client.clone());

    // Combine gRPC and WebSocket servers into one warp service
    let routes = routes::api::routes(shared_redis_client.clone())
        .or(websocket_server)
        .or(grpc_server);

    // Start the warp server
    warp::serve(routes)
        .run(([0, 0, 0, 0], config.server_port))
        .await;
}
