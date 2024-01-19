use tonic::{Request, Response, Status};

use crate::handler::timeline::pb::{EventRequest, EventResponse};
use crate::handler::timeline::pb::timeline_server::{Timeline, TimelineServer};
use crate::service::State;

mod pb {
    tonic::include_proto!("eit.v1");
}

#[derive(Debug, Clone)]
pub struct TimelineHandler {}

impl TimelineHandler {
    pub fn new(state: State) -> Self {
        Self {}
    }

    pub fn into_server(self) -> TimelineServer<Self> {
        TimelineServer::new(self)
    }
}

#[tonic::async_trait]
impl Timeline for TimelineHandler {
    type EventStream = Box<dyn Stream<Item = Result<EventResponse, Status>> + Send + Sync>;

    async fn event(&self, request: Request<EventRequest>) -> Result<Response<Self::EventStream>, Status> {
        // Implementation goes here
        unimplemented!()
    }
}
