use std::sync::Arc;
use axum::{
    extract::State,
    response::IntoResponse,
    routing::get,
    Router,
};
use askama::Template;
use crate::articles::{ArticleRecord, ArticlesRepository};

pub mod worker;
pub mod env;
pub mod feeds;
pub mod articles;

pub fn app(articles_repository: Arc<ArticlesRepository>) -> Router {
    Router::new()
        .route("/", get(handler))
        .with_state(articles_repository)
}

#[derive(Template)]
#[template(path = "articles.html")]
struct ArticlesTemplate {
    articles: Vec<ArticleRecord>,
}

async fn handler(State(articles_repository): State<Arc<ArticlesRepository>>) -> impl IntoResponse {
    let mut articles = articles_repository.find_all().await;
    articles.sort_by(|a, b| b.date.cmp(&a.date));
    let articles = articles.into_iter().take(30).collect();

    ArticlesTemplate { articles }
}
