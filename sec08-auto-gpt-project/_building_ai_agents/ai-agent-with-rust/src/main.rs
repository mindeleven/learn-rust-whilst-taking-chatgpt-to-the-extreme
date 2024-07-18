/// coding along with the tutorial `Building AI Agents with Rust` by Joshua Mo
/// @ https://www.shuttle.rs/blog/2024/05/16/building-ai-content-writer-rust-gpt4o
use axum::{routing::get, Router};
use crate::ai_agents::{
    researcher::Researcher,
    writer::Writer
};

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




async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/", get(hello_world));

    Ok(router.into())
}
