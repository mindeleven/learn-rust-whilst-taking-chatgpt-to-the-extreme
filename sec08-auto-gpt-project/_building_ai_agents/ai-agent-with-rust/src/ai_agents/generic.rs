/// 1. step -> defining a generic interface that all autonomous agents will work with
/// agents we will create:
/// (1) researcher agent ->
///     takes some data from a Google search, 
///     feeds it into ChatGPT,
///     asks ChatGPT to summarize the information
/// (2) a writer agent ->
///     takes the summary and writes an article about it

use async_openai::{config::OpenAIConfig, Client as OpenAIClient};
use crate::helpers::errors::ApiError;

/// defining a generic agent interface
/// the idea behind the generic interface is that it holds all of the relevant methods
/// for the different agents
/// the prompt function alone cannot reference types that it doesn't know about
pub trait Agent {
    fn name(&self) -> String;
    fn client(&self) -> OpenAIClient<OpenAIConfig>;
    fn system_message(&self) -> String;

    // to be given a default implementation later
    async fn prompt(&self, input: &str, data: String) -> Result<String, ApiError>;
}
