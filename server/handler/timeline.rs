use std::net::SocketAddr;

use axum::extract::ws::WebSocket;
use axum::extract::{ConnectInfo, State, WebSocketUpgrade};
use axum::response::IntoResponse;
use axum_extra::{headers, TypedHeader};

use crate::service::timeline::Timeline;

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
async fn handle(mut socket: WebSocket, who: SocketAddr, srv: Timeline) {}
