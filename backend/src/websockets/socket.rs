use warp::ws::{Message, WebSocket};
use warp::Filter;
use futures::{FutureExt, StreamExt};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::db::redis_client::RedisClient;
use crate::models::{User, Post};

pub async fn serve(redis_client: Arc<Mutex<RedisClient>>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("ws")
        .and(warp::ws())
        .and(with_redis_client(redis_client))
        .map(|ws: warp::ws::Ws, redis_client: Arc<Mutex<RedisClient>>| {
            ws.on_upgrade(move |socket| handle_connection(socket, redis_client))
        })
}

fn with_redis_client(
    redis_client: Arc<Mutex<RedisClient>>,
) -> impl Filter<Extract = (Arc<Mutex<RedisClient>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || redis_client.clone())
}

async fn handle_connection(ws: WebSocket, redis_client: Arc<Mutex<RedisClient>>) {
    let (mut tx, mut rx) = ws.split();

    while let Some(result) = rx.next().await {
        match result {
            Ok(msg) => {
                if let Ok(text) = msg.to_str() {
                    // Here you would handle incoming messages, such as saving posts or user actions.
                    // For example, if the message is a new post, you would save it to Redis.
                    if let Ok(post) = serde_json::from_str::<Post>(text) {
                        let mut client = redis_client.lock().await;
                        if let Err(e) = client.save_post(&post).await {
                            eprintln!("Error saving post: {}", e);
                        }
                    }
                }

                // Echo the message back
                if let Err(e) = tx.send(Message::text("Message received")).await {
                    eprintln!("Error sending message: {}", e);
                }
            }
            Err(e) => {
                eprintln!("websocket error: {:?}", e);
            }
        }
    }
}
