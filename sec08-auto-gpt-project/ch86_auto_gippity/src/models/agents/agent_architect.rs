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
use std::fmt::format;
use std::path::Prefix;
use std::time::Duration;

// creating the Solutions Architect
#[derive(Debug)]
struct AgentSolutionArchitect {
    attributes: BasicAgent,
}

impl AgentSolutionArchitect {
    fn new() -> Self {
        let attributes = BasicAgent {
            objective: "Gathers information and design solutions for website development".to_string(),
            position: "Solutions Architect".to_string(),
            state: AgentState::Discovery,
            memory: vec![]
        };

        Self {
            attributes
        }
    }

    // retrieve project scope (which is done by the llm)
    async fn call_project_scope(&mut self, factsheet: &mut FactSheet) -> ProjectScope {
        let msg_context: String = format!("{}", factsheet.project_description);

        let ai_response: ProjectScope = ai_task_request_decoded::<ProjectScope>(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_project_scope),
            print_project_scope
        ).await;

        factsheet.project_scope = Some(ai_response.clone());
        self.attributes.update_state(AgentState::Finished);

        return ai_response;
    }
}