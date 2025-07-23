use axum::Router;
use axum::routing::get;
use tokio::net::TcpListener;
use tokio::signal;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::{Level, error, info};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    let routes = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::very_permissive());

    info!("Server started!");

    tokio::select! {
        res = axum::serve(listener, routes) => res,
        res = signal::ctrl_c() => res,
    }
    .inspect_err(|err| {
        error!("Error in main(): {:?}", err);
    })
}
