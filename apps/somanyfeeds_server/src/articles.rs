use chrono::{DateTime, Utc};
use std::sync::RwLock;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArticleRecord {
    pub title: Option<String>,
    pub link: Option<String>,
    pub content: String,
    pub date: DateTime<Utc>,
    pub feed_name: String,
    pub feed_url: String,
}

pub struct ArticlesRepository {
    articles: RwLock<Vec<ArticleRecord>>,
}

impl ArticlesRepository {
    pub fn new(articles: Vec<ArticleRecord>) -> Self {
        Self {
            articles: RwLock::new(articles),
        }
    }

    pub async fn find_all(&self) -> Vec<ArticleRecord> {
        self.articles.read().unwrap().clone()
    }

    pub async fn replace_all(&self, articles: Vec<ArticleRecord>) {
        let mut lock = self.articles.write().unwrap();
        *lock = articles;
    }
}

impl Default for ArticlesRepository {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}
