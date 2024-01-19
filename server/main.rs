use std::net::SocketAddr;
use std::time::Duration;

use clap::Parser;
use tower_http::trace::*;
use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::util::SubscriberInitExt as _;

mod handler;
mod service;

/// Command-line arguments.
#[derive(Debug, Parser)]
struct Args {
    /// Bounded server port.
    #[arg(short, long, default_value_t = 3000)]
    port: u16,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Tracing.
    let env = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "server=trace,tower_http=trace".into());
    let fmt = tracing_subscriber::fmt::layer().pretty().with_target(false);
    tracing_subscriber::registry().with(fmt).with(env).init();

    // Middlewares.
    // TODO: Headers?
    let tracing_layer = TraceLayer::new_for_grpc()
        .make_span_with(DefaultMakeSpan::new().include_headers(true))
        .on_response(DefaultOnResponse::new().include_headers(true));

    let middlewares = tower::ServiceBuilder::default()
        .timeout(Duration::from_secs(60))
        .layer(tracing_layer)
        .into_inner();

    // Service.
    let state = service::State::connect().await?;
    let timeline = handler::TimelineHandler::new(state.clone()).into_server();
    let status = handler::StatusHandler::new(state.clone()).into_server();

    // Listen.
    let addr = SocketAddr::from(([127, 0, 0, 1], args.port));
    tracing::info!(port = args.port, "server running at {}", addr);
    let server = tonic::transport::Server::builder()
        .layer(middlewares)
        .add_service(timeline)
        .add_service(status);

    server.serve(addr).await?;
    Ok(())
}
