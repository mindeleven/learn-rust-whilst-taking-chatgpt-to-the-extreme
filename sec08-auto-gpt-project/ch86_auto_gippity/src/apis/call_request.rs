use crate::models::general::llm::{ 
    ChatCompletion,
    Message 
};
use reqwest::Client;
use std::fs;

use reqwest::header::{
    HeaderMap,
    HeaderValue
};

// call large language model
// returnig a result that is a string or a dynamic sized error from std
// std::error::Error can hold any type of error the implements the error trait
// advantage: flexibility, different kind of errors can be produeced
// dyn is meant to specify that the Error is a trait object
// dynamic dispatch -> deciding which method to call at runtime
// Send is  a marker trait
// Send implies that ownership of the type implementing Send
// can be safely transfered between threats
pub async fn call_gpt(messages: Vec<Message>) 
    -> Result<String, Box<dyn std::error::Error + Send>> {

    // extract api key information
    // not using dotenv here to protect info from git upload
    let api_key = fs::read_to_string(
        "../../../../../../tmp/chat_gpt/gippity_key_1.txt"
    ).expect("Unable to read api_key file");

    let api_org = fs::read_to_string(
        "../../../../../../tmp/chat_gpt/org_id.txt"
    ).expect("Unable to read org_id file");
    
    // confirm endpoint
    let url: &str = "https://api.openai.com/v1/chat/completions";

    // create headers
    let mut headers = HeaderMap::new();

    // create api key header
    headers.insert(
        "authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key))
            .map_err(|e| -> Box<dyn std::error::Error + Send> {
                Box::new(e)
            })?
    );

    // create api Open AI org header
    headers.insert(
        "OpenAI-Organization",
        HeaderValue::from_str(api_org.as_str())
            .map_err(|e| -> Box<dyn std::error::Error + Send> {
                Box::new(e)
            })?
    );

    // Create client
    let client = Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| -> Box<dyn std::error::Error + Send> {
            Box::new(e)
        })?;

    // create chat completion
    let chat_completion: ChatCompletion = ChatCompletion {
        model: "gpt-4o".to_string(), // gpt-4-turbo
        // model: "gpt-3.5-turbo-1106".to_string(),
        // model: "gpt-3.5".to_string(),
        messages,
        temperature: 0.1
    };

    // troubleshooting, printing the response out
    let res_raw: reqwest::Response = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await
        .unwrap();

    dbg!(res_raw.text().await.unwrap());
    // prints out the following json string:
    /* 
    [src/apis/call_request.rs:84:5] res_raw.text().await.unwrap() = "{\n  \"id\": \"chatcmpl-9jjtlAYEeVCODCLHOh8B1jnl6aEDU\",\n  \"object\": \"chat.completion\",\n  \"created\": 1720688221,\n  \"model\": \"gpt-4o-2024-05-13\",\n  \"choices\": [\n    {\n      \"index\": 0,\n      \"message\": {\n        \"role\": \"assistant\",\n        \"content\": \"Hello! Your test is successful. How can I assist you today?\"\n      },\n      \"logprobs\": null,\n      \"finish_reason\": \"stop\"\n    }\n  ],\n  \"usage\": {\n    \"prompt_tokens\": 22,\n    \"completion_tokens\": 14,\n    \"total_tokens\": 36\n  },\n  \"system_fingerprint\": \"fp_dd932ca5d1\"\n}\n"
    */

    Ok("Temporary placeholder to avoid error messages".to_string())

}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_call_to_open_ai() {
        // cargo test test_call_to_open_ai -- --nocapture

        // 1st, let's create vector of messages
        let message = Message {
            role: "user".to_string(),
            content: "Hi there, this is a test! Please give me a short response.".to_string()
        };

        let messages = vec![message];

        call_gpt(messages).await;

    }
}