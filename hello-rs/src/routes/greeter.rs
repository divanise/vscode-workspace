use askama::Template;
use askama_web::WebTemplate;
use axum::extract::Query;
use axum::response::IntoResponse;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GreeterArgs {
    name: Option<String>,
}

pub async fn greeter(Query(GreeterArgs { name }): Query<GreeterArgs>) -> impl IntoResponse {
    #[derive(Template, WebTemplate)]
    #[template(path = "greeter.html")]
    struct GreeterTemplate {
        greeting: String,
    }

    GreeterTemplate {
        greeting: match name {
            Some(name) => format!("Hello, {}!", name),
            None => "Hello, world!".to_string(),
        },
    }
}
