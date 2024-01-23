use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::Arc;

use anyhow::anyhow;
use axum::extract::ws::Message;
use futures::SinkExt;
use tokio::sync::Mutex;

use crate::internal::events::SubEvent;
use crate::internal::types::{TxStream, UserId};

#[derive(Debug, Default)]
struct RoomInner {
    users: HashMap<UserId, TxStream>,
}

#[derive(Debug, Default)]
pub struct Room {
    inner: Arc<Mutex<RoomInner>>,
}

impl Room {
    pub fn new() -> Self {
        Self::default()
    }

    #[tracing::instrument]
    pub async fn subscribe(&self, id: UserId, tx: TxStream) -> anyhow::Result<()> {
        let mut x = self.inner.lock().await;
        let _ = match x.users.entry(id) {
            Entry::Occupied(_) => Err(anyhow!("attempt to override an occupied UserId")),
            Entry::Vacant(x) => Ok(x.insert(tx)),
        }?;

        Ok(())
    }

    #[tracing::instrument]
    pub async fn unsubscribe(&self, id: UserId) -> anyhow::Result<()> {
        let mut x = self.inner.lock().await;
        let _ = match x.users.entry(id) {
            Entry::Vacant(_) => Err(anyhow!("attempt to clean a vacant UserId")),
            Entry::Occupied(x) => Ok(x.remove()),
        }?;

        Ok(())
    }

    // TODO: Should return FlowControl?
    #[tracing::instrument]
    pub async fn publish(&self, event: SubEvent) -> anyhow::Result<()> {
        let json = serde_json::to_string(&event)?;
        let mut x = self.inner.lock().await;
        for x in x.users.iter_mut() {
            // TODO: Handle errors.
            let message = Message::Text(json.clone());
            let _ = x.1.send(message).await;
        }

        Ok(())
    }
}
