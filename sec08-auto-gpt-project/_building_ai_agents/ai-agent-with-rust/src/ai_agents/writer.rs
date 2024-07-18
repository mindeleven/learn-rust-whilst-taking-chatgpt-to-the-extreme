/// Writer struct representing our Writer Agent

use async_openai::{config::OpenAIConfig, Client as OpenAIClient};
use crate::ai_agents::generic::Agent;
use std::fs;

#[derive(Clone)]
pub struct Writer {
    system: Option<String>,
    client: OpenAIClient<OpenAIConfig>,
}

impl Writer {
    pub fn new() -> Self {
        // let api_key = env::var("OPENAI_API_KEY").unwrap();

        // not using dotenv here to have keys in only one place
        let api_key = fs::read_to_string(
            "../../../../../../tmp/chat_gpt/gippity_key_1.txt"
        ).expect("Unable to read api_key file");

        let config = OpenAIConfig::new().with_api_key(api_key);

        let client = OpenAIClient::with_config(config);
        Self {
            system: None,
            client,
        }
    }
}

impl Agent for Writer {
    fn name(&self) -> String {
        "Writer".to_string()
    }

    fn client(&self) -> OpenAIClient<OpenAIConfig> {
        self.client.clone()
    }

    fn system_message(&self) -> String {
        if let Some(message) = &self.system {
            message.to_owned()
        } else {
            "You are an agent.

        You will receive some context from another agent about some Google results that a user has searched.
        Your job is to research the Internet and to write a high-quality article that a user has written. The article must not appear to be AI written. The article should be SEO optimised without overly compromising the
        quality of the article.

        You are free to be as creative as you wish. However, each paragraph must have the following:
        - The point you are trying to make
        - If there is a follow up action point
        - Why the follow up action point exists (or why the user needs to carry it out)

        Search query:
".to_string()
        }
    }
}
