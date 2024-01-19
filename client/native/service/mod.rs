use crate::service::timeline::Timeline;

mod timeline;

#[derive(Debug, Clone)]
pub struct State {
    timeline: Timeline,
}

impl State {
    pub async fn connect() -> anyhow::Result<Self> {
        Ok(Self {
            timeline: Timeline::new()
        })
    }
}
