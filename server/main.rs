use clap::Parser;
use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::util::SubscriberInitExt as _;

mod service;
mod handler;

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

    Ok(())
}
