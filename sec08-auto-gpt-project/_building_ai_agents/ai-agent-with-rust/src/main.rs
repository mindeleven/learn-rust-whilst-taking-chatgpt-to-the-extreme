/// coding along with the tutorial `Building AI Agents with Rust` by Joshua Mo
/// @ https://www.shuttle.rs/blog/2024/05/16/building-ai-content-writer-rust-gpt4o
use axum::{
    extract::State,
    Json,
    response::IntoResponse,
    routing::{get, post}, 
    Router
};
use crate::ai_agents::{
    researcher::Researcher,
    writer::Writer
};
use serde::{
    Deserialize, 
    Serialize
};
use crate::helpers::errors::ApiError;
use crate::ai_agents::generic::Agent;

mod ai_agents;
mod helpers;

/// implementing the AI agents in our web application
/// 1/ creating an AppState struct that implements Clone 
/// this is a trait bound set by Axum
#[derive(Clone)]
pub struct AppState {
    pub researcher: Researcher,
    pub writer: Writer,
}

impl AppState {
    pub fn new() -> Self {
        // adding researcher and writer to the app state
        let researcher = Researcher::new();
        let writer = Writer::new();

        Self { researcher, writer }
    }
}

/// writing the handler endpoint
#[derive(Deserialize, Serialize)]
pub struct Prompt {
    q: String,
}

/* // -> fn prompt from tutorial
#[axum::debug_handler]
async fn prompt(
    State(state): State<AppState>,
    Json(prompt): Json<Prompt>,
) -> Result<impl IntoResponse, ApiError> {
    let data = state.researcher.prepare_data(&prompt.q).await?;
    let resarcher_result = state.researcher.prompt(&prompt.q, data).await?;
    let writer_result = state.writer.prompt(&prompt.q, res).await?;

    Ok(writer_result)
}
*/

// -> fn prompt from repo
#[axum::debug_handler]
async fn prompt(
    State(state): State<AppState>,
    Json(prompt): Json<Prompt>,
) -> Result<impl IntoResponse, ApiError> {
    let res = state.researcher.prepare_data(&prompt.q).await?;
    let res = state.researcher.prompt(&prompt.q, res).await?;
    let res = state.writer.prompt(&prompt.q, res).await?;

    Ok(res)
}

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let state = AppState::new();

    let router = Router::new()
        .route("/", get(hello_world))
        .route("/prompt", post(prompt))
        .with_state(state);

    Ok(router.into())
}
