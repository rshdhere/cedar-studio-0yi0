mod chat;
mod ui;

use axum::{
    response::{Html, IntoResponse, Json},
    routing::get,
    Router,
};
use chat::ChatState;
use serde_json::json;
use tower_http::trace::TraceLayer;

pub fn app() -> Router {
    let state = ChatState::new();

    Router::new()
        .route("/", get(index))
        .route("/health", get(health))
        .route("/ws", get(chat::ChatState::ws_handler))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
}

async fn index() -> Html<&'static str> {
    Html(ui::INDEX_HTML)
}

async fn health() -> impl IntoResponse {
    Json(json!({ "ok": true }))
}
