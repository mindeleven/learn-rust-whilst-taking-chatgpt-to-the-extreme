use crate::ai_functions::aifunc_architect::{ print_project_scope, print_site_urls };
use crate::helpers::command_line::PrintCommand;
use crate::helpers::general::{ ai_task_request_decoded, check_status_code };
use crate::models::agent_basic::basic_agent::{ AgentState, BasicAgent };
use crate::models::agent_basic::basic_traits::BasicTraits;
use crate::models::agents::agent_traits::{
    FactSheet,
    ProjectScope,
    SpecialFunctions
};