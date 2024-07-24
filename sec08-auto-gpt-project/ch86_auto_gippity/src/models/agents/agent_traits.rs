// special function trait
// creating special traits for agents
// managing agents only will work with agents who have these special traits

use crate::models::agent_basic::basic_agent::BasicAgent;
// use async_trait::async_trait; // most likely not needed in this Rust version
use serde::{ Deserialize, Serialize };
use std::fmt::Debug;

// how does the schema for the api endpoints that we expect look like?
// do get a better understanding of what this struct is doing, have a look at
// ai_functions::aifunc_backend::print_rest_api_endpoints
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
// fact sheet can be passed from managing agent to the subordinated agents
// and get updated by these agents (solutions + backend)
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct FactSheet {
    pub project_description: String,
    // when we first create the project the scope will be None
    // when we get the project it will be some
    pub project_scope: Option<ProjectScope>,
    pub external_urls: Option<Vec<String>>,
    pub backend_code: Option<String>,
    pub api_endpoint_schema: Option<Vec<RouteObject>>,
}

// special powers for the agents
pub trait SpecialFunctions: Debug { // trait needs to have Debug
    // function used by the managing agent
    // so that manager can get attributes from the Agents
    fn get_attributes_from_agent(&self) -> &BasicAgent;
    
    // This function will allow agents to execute their logic
    // managing agent will be able to call on the agents to execute
    // whatever they need to do
    async fn execute(&mut self, factsheet: &mut FactSheet) 
        -> Result<(), Box<dyn std::error::Error>>;
}
