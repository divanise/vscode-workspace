use tokio::net::TcpListener;
use tokio::signal;
use tracing::{Level, error, info};

mod routes;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    let routes = routes::new();

    info!("Server started!");

    tokio::select! {
        res = axum::serve(listener, routes) => res,
        res = signal::ctrl_c() => res,
    }
    .inspect_err(|err| {
        error!("Error in main(): {:?}", err);
    })
}
