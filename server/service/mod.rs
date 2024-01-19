pub use crate::service::timeline::Timeline;

mod timeline;

#[derive(Debug, Clone)]
pub struct State {
    timeline: Timeline,
}

impl State {
    pub async fn connect() -> anyhow::Result<Self> {
        Ok(Self {
            timeline: Timeline::connect().await?,
        })
    }
}

macro_rules! impl_di {
    ($($f:ident: $t:ty),+) => {$(
        impl axum::extract::FromRef<State> for $t {
            fn from_ref(state: &State) -> Self {
                state.$f.clone()
            }
        }
    )+};
}

impl_di!(timeline: Timeline);
