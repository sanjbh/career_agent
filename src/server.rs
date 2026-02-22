use std::sync::Arc;

use async_openai::types::chat::ChatCompletionRequestMessage;
use axum::{
    Json, Router,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};

use crate::{agent::Agent, evaluator::Evaluator};

pub struct AppState {
    agent: Agent,
    evaluator: Evaluator,
}

#[derive(Deserialize)]
pub struct ChatRequest {
    message: String,
    history: Vec<ChatCompletionRequestMessage>,
}

#[derive(Serialize)]
pub struct ChatResponse {
    reply: String,
    feedback: String,
    rerun: bool,
}

pub async fn get_ui() -> Result<impl IntoResponse, AppError> {
    let content = tokio::fs::read_to_string("static/index.html").await?;
    Ok(Html(content))
}

pub async fn post_chat(Json(payload): Json<ChatRequest>) -> Json<ChatResponse> {}

pub fn router(agent: Agent, evaluator: Evaluator) -> Router {
    let shared_state = Arc::new(AppState { agent, evaluator });

    Router::new()
        .route("/", get(get_ui))
        .route("/chat", post(post_chat))
        .with_state(shared_state)
}

pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string()).into_response()
    }
}

impl<E: Into<anyhow::Error>> From<E> for AppError {
    fn from(value: E) -> Self {
        AppError(value.into())
    }
}
