pub use crate::service::status::StatusService;

mod status;

#[derive(Debug, Clone)]
pub struct State {
    status: StatusService,
}

impl State {
    pub async fn connect() -> anyhow::Result<Self> {
        Ok(Self {
            status: StatusService::connect().await?,
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

impl_di!(status: StatusService);
