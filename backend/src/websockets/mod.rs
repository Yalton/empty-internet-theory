pub mod socket;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use warp::ws::{Message, WebSocket};
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;
use crate::models::{User, Post};
use crate::db::redis_client::RedisClient;

// Define a type alias for a Users HashMap where the key is a user ID and the value is a sender channel
type Users = Arc<Mutex<HashMap<String, mpsc::UnboundedSender<Result<Message, warp::Error>>>>>;

// Function to handle incoming WebSocket connections
pub async fn user_connected(ws: WebSocket, user_id: String, users: Users, redis_client: RedisClient) {
    // Split the WebSocket into a sender and receiver
    let (user_ws_tx, mut user_ws_rx) = ws.split();

    // Use an unbounded channel to handle buffering and flushing of messages to the WebSocket...
    let (tx, rx) = mpsc::unbounded_channel();
    let rx = tokio_stream::wrappers::UnboundedReceiverStream::new(rx);

    // Create a task that sends messages to the user
    tokio::task::spawn(rx.forward(user_ws_tx).map(|result| {
        if let Err(e) = result {
            eprintln!("websocket send error: {}", e);
        }
    }));

    // Save the sender in the users list
    users.lock().unwrap().insert(user_id.clone(), tx);

    // Receive messages from the user
    while let Some(result) = user_ws_rx.next().await {
        match result {
            Ok(msg) => {
                if msg.is_text() {
                    handle_message(&user_id, &msg.to_str().unwrap(), &users, &redis_client).await;
                } else if msg.is_close() {
                    break;
                }
            }
            Err(e) => {
                eprintln!("websocket error(uid={}): {}", user_id, e);
                break;
            }
        }
    }

    // User disconnected
    user_disconnected(&user_id, &users).await;
}

// Function to handle messages received from a user
async fn handle_message(user_id: &str, message: &str, users: &Users, redis_client: &RedisClient) {
    // Here you would handle different message types (e.g., new post, delete post, etc.)
    // For simplicity, we'll just echo the message back to the user
    if let Some(sender) = users.lock().unwrap().get(user_id) {
        if let Err(_send_error) = sender.send(Ok(Message::text(message))) {
            // Handle the error, e.g., by logging it or removing the user from the users list
        }
    }

    // Example: Save a new post to Redis
    if message.starts_with("post:") {
        let content = message.strip_prefix("post:").unwrap_or_default();
        let post = Post::new(0, user_id.parse().unwrap(), content.to_string()); // ID should be generated properly
        if let Err(e) = redis_client.save_post(&post) {
            eprintln!("Error saving post: {}", e);
        }
    }
}

// Function to handle a user disconnecting
async fn user_disconnected(user_id: &str, users: &Users) {
    eprintln!("user disconnected: {}", user_id);
    // Remove the user from the users HashMap
    users.lock().unwrap().remove(user_id);
}
