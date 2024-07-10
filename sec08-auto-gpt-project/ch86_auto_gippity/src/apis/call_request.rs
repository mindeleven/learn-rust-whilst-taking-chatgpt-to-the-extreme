use crate::models::general::llm::{ Message };
use reqwest::Client;
use std::fs;

use reqwest::header::{
    HeaderMap,
    HeaderValue
};

// call large language model
pub async fn call_gpt(messages: Vec<Message>) {

    // extract api key information
    // not using dotenv here to protect info from git upload
    let api_key = fs::read_to_string(
        "../../../../../../tmp/chat_gpt/gippity_key_1.txt"
    ).expect("Unable to read api_key file");

    let api_org = fs::read_to_string(
        "../../../../../../tmp/chat_gpt/org_id.txt"
    ).expect("Unable to read org_id file");
    
    // confirm endpoint
    let url: &str = "https://api.openai.com/";

    // create headers
    let mut headers = HeaderMap::new();

    // create api key header
    headers.insert(
        "authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap()
    );

    // create api Open AI org header
    headers.insert(
        "OpenAI-Organization",
        HeaderValue::from_str(api_org.as_str()).unwrap()
    );

}