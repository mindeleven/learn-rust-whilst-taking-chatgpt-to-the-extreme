/// coding along with the tutorial `Building AI Agents with Rust` by Joshua Mo
/// @ https://www.shuttle.rs/blog/2024/05/16/building-ai-content-writer-rust-gpt4o
/// 
/// this file is very much copy/paste from the tutorial for learning purposes
/// 
/// defining custom error types for error propagation
/// if we want the error to be propagated we can use this enum as the error return type,
/// then use error propagation
use async_openai::error::OpenAIError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("OpenAI error: {0}")]
    OpenAI(#[from] OpenAIError),
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("De/serialization error: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, body) = match self {
            Self::OpenAI(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            Self::Reqwest(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            Self::SerdeJson(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
        };

        println!("An error happened: {body}");

        (status, body).into_response()
    }
}
