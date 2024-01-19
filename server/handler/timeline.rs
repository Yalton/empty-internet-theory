use tokio::sync::mpsc::unbounded_channel;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tonic::{Request, Response, Status, Streaming};

use crate::handler::timeline::pb::timeline_server::{Timeline, TimelineServer};
use crate::handler::timeline::pb::{EventRequest, EventResponse};
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
    type EventStream = UnboundedReceiverStream<Result<EventResponse, Status>>;

    async fn event(
        &self,
        request: Request<Streaming<EventRequest>>,
    ) -> Result<Response<Self::EventStream>, Status> {
        let (_, rx) = unbounded_channel();
        let stream = UnboundedReceiverStream::new(rx);
        Ok(Response::new(stream))
    }
}
