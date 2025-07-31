use tokio::net::TcpListener;
use tokio::signal;
use tracing::{error, info};

use crate::config::Config;

mod config;
mod routes;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config_path = Config::path();
    let config = Config::load(&config_path).unwrap_or_else(|_| {
        panic!(
            "Failed to load configuration from {}",
            config_path.display()
        )
    });

    tracing_subscriber::fmt()
        .with_max_level(config.log_level())
        .init();

    info!(
        "Loaded configuration from {:?}: {:#?}",
        config_path.display(),
        config
    );

    let http_addr = (config.http_host(), config.http_port());
    let listener = TcpListener::bind(http_addr).await?;
    let routes = routes::new();

    tokio::select! {
        res = axum::serve(listener, routes) => res,
        res = signal::ctrl_c() => res,
    }
    .inspect_err(|err| {
        error!("Error in main(): {:?}", err);
    })
}
