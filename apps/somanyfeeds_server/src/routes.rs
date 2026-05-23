use crate::articles::{ArticleRecord, ArticlesRepository};
use askama::Template;
use axum::{Router, extract::State, response::IntoResponse, routing::get};
use chrono_tz::America::Denver;
use std::sync::Arc;
use tower_http::{services::ServeDir, trace::TraceLayer};

pub struct RouterSettings {
    pub public_path: String,
}

pub fn router(articles_repository: Arc<ArticlesRepository>, settings: RouterSettings) -> Router {
    Router::new()
        .route("/", get(handler))
        .fallback_service(ServeDir::new(settings.public_path))
        .layer(TraceLayer::new_for_http())
        .with_state(articles_repository)
}

#[derive(Template)]
#[template(path = "articles.html")]
struct ArticleListTemplate {
    articles: Vec<ArticleView>,
}

#[allow(dead_code)]
struct ArticleView {
    pub title: Option<String>,
    pub link: Option<String>,
    pub content: String,
    pub date: String,
    pub feed_name: String,
    pub feed_url: String,
}

impl From<ArticleRecord> for ArticleView {
    fn from(record: ArticleRecord) -> Self {
        Self {
            title: record.title,
            link: record.link,
            content: record.content,
            date: record
                .date
                .with_timezone(&Denver)
                .format("%b %d '%y @ %H:%M")
                .to_string(),
            feed_name: record.feed_name,
            feed_url: record.feed_url,
        }
    }
}

async fn handler(State(articles_repository): State<Arc<ArticlesRepository>>) -> impl IntoResponse {
    let mut articles = articles_repository.find_all().await;
    articles.sort_by(|a, b| b.date.cmp(&a.date));

    let mut articles: Vec<ArticleView> = articles.into_iter().map(ArticleView::from).collect();

    articles.insert(
        0,
        ArticleView {
            title: Some("About".to_string()),
            link: None,
            content: crate::about::ABOUT_ARTICLE_CONTENT.to_string(),
            date: "".to_string(),
            feed_name: "About".to_string(),
            feed_url: "".to_string(),
        },
    );

    ArticleListTemplate { articles }
}
