mod api;
mod html;

use std::future::Future;
use std::sync::Arc;

use axum::Router;
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
        .merge(html::routes())
        .nest("/api", api::routes())
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::very_permissive())
        .with_state(Arc::new(services))
}
