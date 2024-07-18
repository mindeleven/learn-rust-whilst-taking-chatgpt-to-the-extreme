/// Researcher struct will hold the data and methods for 
/// (1) querying the Serper API 
/// and (2) using the data to prompt a model

use async_openai::{config::OpenAIConfig, Client as OpenAIClient};
use crate::ai_agents::generic::Agent;

#[derive(Clone)]
pub struct Researcher {
    http_client: reqwest::Client,
    system: Option<String>,
    openai_client: OpenAIClient<OpenAIConfig>,
}

// implementing the Agent trait for the Researcher struct
impl Agent for Researcher {
    // boilerplate name() function
    fn name(&self) -> String {
        "Researcher".to_string()
    }

    // boilerplate client() function
    fn client(&self) -> OpenAIClient<OpenAIConfig> {
        self.openai_client.clone()
    }

    fn system_message(&self) -> String {
        if let Some(message) = &self.system {
            message.to_owned()
        } else {
            "You are an agent.

        You will receive a question that may be quite short or does not have much context.
        Your job is to research the Internet and to return with a high-quality summary to the user, assisted by the provided context.
        The provided context will be in JSON format and contains data about the initial Google results for the website or query.

        Be concise.

        Question:
        ".to_string()
        }
    }

}


