use axum::Router;
use axum::response::Html;
use axum::routing::get;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

const INDEX_HTML: &str = include_str!("index.html");

pub fn new() -> Router {
    Router::new()
        .route("/", get(Html(INDEX_HTML)))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::very_permissive())
}
