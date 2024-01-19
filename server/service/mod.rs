#[derive(Debug, Clone)]
pub struct State {}

impl State {
    pub async fn connect() -> anyhow::Result<Self> {
        Ok(Self {})
    }
}
