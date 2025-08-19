use std::sync::Arc;

use axum::http::StatusCode;
use axum::{Json, Router};

use super::Services;

pub fn routes<S>() -> Router<Arc<S>>
where
    S: Services,
    S: Send + Sync + 'static,
{
    let v1 = v1::routes();

    Router::new().nest("/v1", v1).fallback((
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({ "status": "Not Found" })),
    ))
}

mod v1 {
    use std::sync::Arc;

    use axum::Router;
    use axum::extract::{Json, State};
    use axum::http::StatusCode;
    use axum::response::IntoResponse;
    use axum::routing::post;
    use serde::Deserialize;

    use super::Services;

    #[derive(Deserialize)]
    struct GreetArgs {
        name: Option<String>,
    }

    async fn greet<S: Services>(
        State(services): State<Arc<S>>,
        Json(GreetArgs { name }): Json<GreetArgs>,
    ) -> impl IntoResponse {
        let greeting = services.greet(name).await;

        (
            StatusCode::OK,
            axum::Json(serde_json::json!({ "greeting": greeting })),
        )
    }

    pub fn routes<S>() -> Router<Arc<S>>
    where
        S: Services,
        S: Send + Sync + 'static,
    {
        Router::new().route("/greet", post(greet))
    }
}
