use std::net::SocketAddr;
use std::time::Duration;

use axum::{error_handling::*, routing::*};
use clap::Parser;
use tower_http::ServiceBuilderExt as _;
use tower_http::{cors::*, request_id::*, trace::*};
use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::util::SubscriberInitExt as _;

mod handler;
mod middleware;
mod service;

/// Command-line arguments.
#[derive(Debug, Parser)]
struct Args {
    /// Bounded server port.
    #[arg(short, long, default_value_t = 3000)]
    port: u16,
    #[arg(short, long, default_value = "redis://127.0.0.1/")]
    redis: String,
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
    // TODO: Move to middleware::instantiate_stack().
    // TODO: Client failures as failures.
    let error_layer = HandleErrorLayer::new(middleware::handle_box_error);
    let tracing_layer = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().include_headers(true))
        .on_response(DefaultOnResponse::new().include_headers(true));
    let cors_layer = CorsLayer::permissive();

    let middlewares = tower::ServiceBuilder::default()
        .layer(cors_layer)
        .layer(error_layer)
        .timeout(Duration::from_secs(60))
        .compression()
        // .request_body_limit()
        .set_x_request_id(MakeRequestUuid)
        .layer(tracing_layer)
        .propagate_x_request_id();

    // Service.
    let state = service::State::connect().await?;
    let app = Router::new()
        .route("/status", get(handler::status::check))
        .route("/account/up", post(handler::account::sign_up))
        .route("/account/in", post(handler::account::sign_in))
        .route("/onetime/", post(handler::account::visit))
        .route("/ws", get(handler::timeline::subscribe))
        .fallback(handler::fallback)
        .layer(middlewares)
        .with_state(state);

    // Listen.
    let addr = SocketAddr::from(([127, 0, 0, 1], args.port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!(port = args.port, "server running at {}", addr);
    let app = app.into_make_service_with_connect_info::<SocketAddr>();
    axum::serve(listener, app).await?;

    Ok(())
}
