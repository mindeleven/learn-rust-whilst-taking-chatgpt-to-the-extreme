/// An example Chat Completions API call can be found at
/// https://platform.openai.com/docs/guides/text-generation/chat-completions-api

use serde::{
    Serialize, 
    Deserialize
};

/// recreating the message example with a struct
#[derive(Debug, Serialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String
}

#[derive(Debug, Serialize, Clone)]
pub struct ChatCompletion {
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: f32
}

#[derive(Debug, Deserialize)]
pub struct APIMessage {
    content: String
}

#[derive(Debug, Deserialize)]
pub struct APIChoice {
    message: APIMessage
}


#[derive(Debug, Deserialize)]
pub struct APIResponse {
    choices: Vec<APIChoice>
}