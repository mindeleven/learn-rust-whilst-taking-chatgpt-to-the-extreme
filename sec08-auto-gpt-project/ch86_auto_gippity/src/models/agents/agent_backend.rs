// Building out the backend agent
use crate::ai_functions::aifunc_backend::{
    print_backend_webserver_code, print_fixed_code, print_improved_webserver_code,
    print_rest_api_endpoints,
};
use crate::helpers::general::{
    check_status_code, read_code_template_contents, 
    save_api_endpoints, save_backend_code
};

use crate::helpers::command_line::PrintCommand;
use crate::helpers::general::ai_task_request;
use crate::models::agent_basic::basic_agent::{ AgentState, BasicAgent };
use crate::models::agents::agent_traits::{ FactSheet, RouteObject, SpecialFunctions };

use async_trait::async_trait;
use reqwest::Client;
use std::process::{ Command, Stdio };
use std::time::Duration;
use tokio::time;

#[derive(Debug)]
struct AgentBackendDeveloper {
    attributes: BasicAgent,
    bug_errors: Option<String>,
    bug_count: u8
}

impl AgentBackendDeveloper {
    pub fn new() -> Self {
        let attributes = BasicAgent {
            objective: "Gathers information and design solutions for website development".to_string(),
            position: "Backend Developer".to_string(),
            state: AgentState::Discovery,
            memory: vec![]
        };

        Self {
            attributes,
            bug_errors: None,
            bug_count: 0
        }
    }

    async fn call_initial_backend_code(&self, factsheet: &mut FactSheet) {

        let code_template_str: String = read_code_template_contents();

        // concatenate instruction
        let mut msg_context: String = format!(
            "CODE TEMPLATE: {} \n PROJECT DESCRIPTION: {} \n",
            code_template_str, factsheet.project_description
        );

        let ai_response = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_backend_webserver_code),
            print_backend_webserver_code
        ).await;

        save_backend_code(&ai_response);
        factsheet.backend_code = Some(ai_response);

    }

    async fn call_improved_backend_code(&self, factsheet: &mut FactSheet) {
        // concatenate instruction
        // improved backend code is stored in memory in the factsheet
        let mut msg_context: String = format!(
            "CODE TEMPLATE: {:?} \n PROJECT DESCRIPTION: {:?} \n",
            factsheet.backend_code, factsheet
        );

        let ai_response = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_improved_webserver_code),
            print_improved_webserver_code
        ).await;

        save_backend_code(&ai_response);
        factsheet.backend_code = Some(ai_response);

    }
}