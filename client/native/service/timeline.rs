mod pb {
    tonic::include_proto!("eit.v1");
}

#[derive(Debug, Clone)]
pub struct Timeline {}

impl Timeline {
    pub fn new() -> Self {
        Self {}
    }
}
