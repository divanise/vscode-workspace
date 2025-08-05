use axum::Router;
use axum::routing::get;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

mod greeter;

pub fn new() -> Router {
    Router::new()
        .route("/", get(greeter::greeter))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::very_permissive())
}
