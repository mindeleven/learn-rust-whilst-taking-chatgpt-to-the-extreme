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

    // Create client
    let client = Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();

    // create chat completion
    let chat_completion: ChatCompletion = ChatCompletion {
        model: "gpt-4".to_string(),
        messages,
        temperature: 0.1
    };

    // troubleshooting
    let res_raw = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await
        .unwrap();

    dbg!(res_raw.text().await.unwrap());

}