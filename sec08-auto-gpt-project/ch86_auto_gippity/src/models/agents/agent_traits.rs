// special function trait
// creating special traits for agents
// managing agents only will work with agents who have these special traits

use crate::models::agent_basic::basic_agent::BasicAgent;
use async_trait::async_trait;
use serde::{ Deserialize, Serialize };
use std::fmt::Debug;

// building the fact sheet 
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct FactSheet {
    pub project_description: String,
    pub project_scope: String,
    pub external_urls: String,
    pub backend_code: String,
    pub api_endpoint_schema: String,
}