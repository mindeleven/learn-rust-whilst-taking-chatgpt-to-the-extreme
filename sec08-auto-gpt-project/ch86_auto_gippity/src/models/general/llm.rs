/// An example Chat Completions API call can be found at
/// https://platform.openai.com/docs/guides/text-generation/chat-completions-api

use serde::{
    Serialize, 
    Deserialize
};

/// recreating the message example with a struct
#[derive(Debug, Serialize, Clone)]
pub struct Message {
    role: String,
    content: String
}

#[derive(Debug, Serialize, Clone)]
pub struct ChatCompletion {
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: f32
}