use crate::ai_functions::aifunc_architect::{ print_project_scope, print_site_urls };
use crate::helpers::command_line::PrintCommand;
// bringing in the ability to go and request a llm, to give us information and then
// decode this information into the struct or type we want
// plus to check the status code of any urls
use crate::helpers::general::{ ai_task_request_decoded, check_status_code };
use crate::models::agent_basic::basic_agent::{ AgentState, BasicAgent };
use crate::models::agent_basic::basic_traits::BasicTraits;
use crate::models::agents::agent_traits::{
    FactSheet,
    ProjectScope,
    SpecialFunctions
};

use async_trait::async_trait;
use reqwest::Client;
use std::time::Duration;

// creating the Solutions Architect