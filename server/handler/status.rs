use tonic::{Request, Response, Status};

use crate::handler::status::pb::info_server::{Info, InfoServer};
use crate::handler::status::pb::{StatusRequest, StatusResponse};
use crate::service::State;

mod pb {
    tonic::include_proto!("eit.v1");
}

#[derive(Debug, Clone)]
pub struct StatusHandler {}

impl StatusHandler {
    pub fn new(state: State) -> Self {
        Self {}
    }

    pub fn into_server(self) -> InfoServer<Self> {
        InfoServer::new(self)
    }
}

#[tonic::async_trait]
impl Info for StatusHandler {
    async fn status(
        &self,
        request: Request<StatusRequest>,
    ) -> Result<Response<StatusResponse>, Status> {
        todo!()
    }
}
