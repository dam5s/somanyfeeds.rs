use axum::{
    extract::Query,
    response::IntoResponse,
    routing::get,
    Router,
};
use askama::Template;
use serde::Deserialize;

pub mod worker;
pub mod env;
pub mod feeds;
pub mod articles;

pub fn app() -> Router {
    Router::new().route("/", get(handler))
}

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate {
    name: String,
}

#[derive(Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler(Query(params): Query<HelloParams>) -> impl IntoResponse {
    let name = params.name.unwrap_or_else(|| "World".to_string());
    HelloTemplate { name }
}
