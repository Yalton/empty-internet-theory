pub use status::StatusHandler;

mod status;
mod timeline;

#[tonic::async_trait]
pub trait CheckStatus {
    /// Used as a reason of the bad status check.
    /// TODO: Infallible as a default error type.
    type Error;

    /// Returns `Ok()` if the service is healthy, reason otherwise.
    async fn check(&self) -> Result<(), Self::Error> {
        Ok(())
    }
}
