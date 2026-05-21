use axum::{
    routing::get,
    response::Html,
    Router,
};

pub fn app() -> Router {
    Router::new().route("/", get(handler))
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello World</h1>")
}
