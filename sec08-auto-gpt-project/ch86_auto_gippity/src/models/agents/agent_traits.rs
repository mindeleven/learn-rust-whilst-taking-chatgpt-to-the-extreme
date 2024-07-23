// special function trait
// creating special traits for agents
// managing agents only will work with agents who have these special traits

use crate::models::agent_basic::basic_agent::BasicAgent;
use async_trait::async_trait;
use serde::{ Deserialize, Serialize };
use std::fmt::Debug;

// how does the schema for the api endpoints that we expect look like?
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct RouteObject {
    is_route_dynamic: String,
    method: String,
    request_body: serde_json::Value,
    route: String,
}

// the struct we expect the llm to return ??? (needs clarification)
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
pub struct ProjectScope {
    is_crud_required: bool,
    is_user_login_and_logout: bool,
    is_external_urls_required: bool,
}

// building the fact sheet 
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct FactSheet {
    pub project_description: String,
    // when we first create the project the scope will be None
    // when we get the project it will be some
    pub project_scope: Option<ProjectScope>,
    pub external_urls: String,
    pub backend_code: String,
    pub api_endpoint_schema: String,
}