mod greeter;

use std::future::Future;
use std::sync::Arc;

use axum::Router;
use axum::routing::get;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

pub trait Services {
    fn greet(&self, name: Option<String>) -> impl Future<Output = String> + Send;
}

pub fn new<S>(services: S) -> Router
where
    S: Services,
    S: Send + Sync + 'static,
{
    Router::new()
        .route("/", get(greeter::greeter))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::very_permissive())
        .with_state(Arc::new(services))
}
