use std::net::SocketAddr;

use axum::extract::ws::WebSocket;
use axum::extract::{ConnectInfo, State, WebSocketUpgrade};
use axum::response::IntoResponse;
use axum_extra::{headers, TypedHeader};
use futures::stream::StreamExt;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::service::timeline::Timeline;

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum EventSub {
    Tweet { content: String },
    Like { id: String },
    Report { id: String },
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum EventPub {
    Tweet {
        id: String,
        timestamp: f32,
        user_id: String,
        content: String,
    },

    Retweet {
        id: String,
        timestamp: f32,
        tweet_id: String,
        user_id: String,
        content: String,
    },

    /// Left or kicked.
    Ban {
        timestamp: f32,
        user_id: String,
        reason: String,
    },
}

pub async fn subscribe(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(timeline): State<Timeline>,
) -> impl IntoResponse {
    let user_agent = user_agent
        .map(|x| x.0.to_string())
        .unwrap_or_else(|| "Unknown browser".to_owned());

    tracing::info!("`{user_agent}` at {addr} connected.");
    ws.on_upgrade(move |socket| handle(socket, addr, timeline))
}

// https://github.com/tokio-rs/axum/blob/main/examples/websockets/src/main.rs
async fn handle(mut socket: WebSocket, who: SocketAddr, srv: Timeline) {
    let (mut sender, mut receiver) = socket.split();

    while let Some(Ok(message)) = receiver.next().await {}
}
