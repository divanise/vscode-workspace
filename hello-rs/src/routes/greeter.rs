use std::sync::Arc;

use askama::Template;
use askama_web::WebTemplate;
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use serde::Deserialize;

use super::Services;

#[derive(Deserialize)]
pub struct GreeterArgs {
    name: Option<String>,
}

pub async fn greeter<S: Services>(
    Query(GreeterArgs { name }): Query<GreeterArgs>,
    State(services): State<Arc<S>>,
) -> impl IntoResponse {
    #[derive(Template, WebTemplate)]
    #[template(path = "greeter.html")]
    struct GreeterTemplate {
        greeting: String,
    }

    let greeting = services.greet(name).await;

    GreeterTemplate { greeting }
}
