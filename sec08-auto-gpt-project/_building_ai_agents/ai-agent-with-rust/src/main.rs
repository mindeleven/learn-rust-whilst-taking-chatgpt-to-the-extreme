/// coding along with the tutorial `Building AI Agents with Rust` by Joshua Mo
/// @ https://www.shuttle.rs/blog/2024/05/16/building-ai-content-writer-rust-gpt4o
use axum::{routing::get, Router};

mod ai_agents;
mod helpers;


async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/", get(hello_world));

    Ok(router.into())
}
