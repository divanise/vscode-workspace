use std::sync::Arc;

use askama::Template;
use askama_web::WebTemplate;
use axum::Router;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use serde::Deserialize;

use super::Services;

#[derive(Deserialize)]
struct GreeterArgs {
    name: Option<String>,
}

async fn greeter<S: Services>(
    State(services): State<Arc<S>>,
    Query(GreeterArgs { name }): Query<GreeterArgs>,
) -> impl IntoResponse {
    #[derive(Template, WebTemplate)]
    #[template(path = "greeter.html")]
    struct GreeterTemplate {
        greeting: String,
    }

    let greeting = services.greet(name).await;

    GreeterTemplate { greeting }
}

pub fn routes<S>() -> Router<Arc<S>>
where
    S: Services,
    S: Send + Sync + 'static,
{
    Router::new()
        .route("/", get(greeter))
        .fallback((StatusCode::NOT_FOUND, "Not Found"))
}
