use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;

use anyhow::anyhow;
use tokio::sync::Mutex;

use crate::handler::status::CheckStatus;
use crate::internal::events::SubEvent;
use crate::internal::rooms::Room;
use crate::internal::types::{RoomId, TxStream, UserId};

#[derive(Debug, Default)]
struct TimelineInner {
    rooms: HashMap<RoomId, Room>,
}

#[derive(Debug, Default, Clone)]
pub struct Timeline {
    inner: Arc<Mutex<TimelineInner>>,
}

impl Timeline {
    pub fn new() -> Self {
        Self::default()
    }

    #[tracing::instrument]
    pub async fn subscribe(&self, room: RoomId, id: UserId, tx: TxStream) -> anyhow::Result<()> {
        let inner = self.inner.lock().await;
        match inner.rooms.get(&room) {
            None => Err(anyhow!("attempt to subscribe to the unknown room")),
            Some(room) => room.subscribe(id, tx).await,
        }
    }

    #[tracing::instrument]
    pub async fn unsubscribe(&self, room: RoomId, id: UserId) -> anyhow::Result<()> {
        let inner = self.inner.lock().await;
        match inner.rooms.get(&room) {
            None => Err(anyhow!("attempt to unsubscribe from the unknown room")),
            Some(room) => room.unsubscribe(id).await,
        }

        // TODO: Publish ban with *deleted his account*.
    }

    #[tracing::instrument]
    pub async fn publish(&self, room: RoomId, event: SubEvent) -> anyhow::Result<()> {
        let inner = self.inner.lock().await;
        match inner.rooms.get(&room) {
            None => Err(anyhow!("attempt to publish to the unknown room")),
            Some(room) => room.publish(event).await,
        }
    }
}

impl CheckStatus for Timeline {
    type Error = Infallible;
}
