use std::net::SocketAddr;

use axum::extract::ws::{Message, WebSocket};
use axum::extract::{ConnectInfo, State, WebSocketUpgrade};
use axum::response::IntoResponse;
use axum_extra::{headers, TypedHeader};
use futures::stream::StreamExt;

use crate::internal::events::PubEvent;
use crate::service::account::Account;
use crate::service::timeline::Timeline;

pub async fn subscribe(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(account): State<Account>,
    State(timeline): State<Timeline>,
) -> impl IntoResponse {
    let user_agent = user_agent
        .map(|x| x.0.to_string())
        .unwrap_or_else(|| "Unknown browser".to_owned());

    tracing::info!("`{user_agent}` at {addr} connected.");
    ws.on_upgrade(move |socket| handle(socket, addr, account, timeline))
}

// https://github.com/tokio-rs/axum/blob/main/examples/websockets/src/main.rs
async fn handle(
    mut socket: WebSocket,
    socket_addr: SocketAddr,
    account: Account,
    timeline: Timeline,
) {
    let (tx, mut rx) = socket.split();
    let mut tx = Some(tx);

    // Give up tx to timeline, use rx only?

    while let Some(message) = rx.next().await {
        match message {
            Ok(x) => match x {
                Message::Text(x) => {
                    /* Invalid: Close or Skip? */
                    let event = match serde_json::from_str::<PubEvent>(x.as_str()) {
                        Ok(x) => x,
                        Err(_) => continue,
                    };

                    let (user, room) = match account.auth(&event.token()).await {
                        Ok(x) => x,
                        Err(_) => continue,
                    };
                }
                Message::Binary(_) => { /* Unsupported: Close? */ }
                Message::Ping(_) | Message::Pong(_) => {}
                Message::Close(_) => {}
            },

            Err(_) => { /* Error: Close? */ }
        }
    }

    // TODO: Last message received, update token data and unsubscribe.
}
