// Building out the backend agent
use crate::ai_functions::aifunc_backend::{
    print_backend_webserver_code, print_fixed_code, print_improved_webserver_code,
    print_rest_api_endpoints
};
use crate::helpers::general::{
    check_status_code, read_code_template_contents, read_exec_main_contents,
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

    async fn call_fix_code_bugs(&self, factsheet: &mut FactSheet) {
        // concatenate instruction
        // improved backend code is stored in memory in the factsheet
        let mut msg_context: String = format!(
            "BROKEN_CODE: {:?} \n ERROR_BUGS: {:?} \n
             THIS FUNCTION ONLY OUTPUTS CODE. JUST OUTPUT THE CODE.",
            factsheet.backend_code, self.bug_errors
        );

        let ai_response = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_fixed_code),
            print_fixed_code
        ).await;

        save_backend_code(&ai_response);
        factsheet.backend_code = Some(ai_response);

    }
    
    async fn call_extract_rest_api_endpoints(&self) -> String {
        // getting code that's inside main.rs file
        // getting it from file instead of factsheet because from factsheet 
        // means less expensive llm connections (-> cheaper)
        let backend_code = read_exec_main_contents();

        // structure message context
        let msg_context: String = format!("CODE_INPUT: {}", backend_code);

        let ai_response = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_rest_api_endpoints),
            print_rest_api_endpoints
        ).await;
        
        ai_response
    }

}

// implementing special functions for the backend developer
#[async_trait]
impl SpecialFunctions for AgentBackendDeveloper {

    fn get_attributes_from_agent(&self) -> &BasicAgent {
        // returning all the information about this agent
        &self.attributes
    }

    async fn execute(
        &mut self, 
        factsheet: &mut FactSheet
    ) -> Result<(), Box<dyn std::error::Error>> {

        while self.attributes.state != AgentState::Finished {
            
            match self.attributes.state {
                AgentState::Discovery => {
                    self.call_initial_backend_code(factsheet).await;
                    self.attributes.state = AgentState::Working;
                    continue;
                },
                AgentState::Working => {
                    if self.bug_count == 0 {
                        self.call_improved_backend_code(factsheet).await;
                        self.attributes.state = AgentState::UnitTesting;
                        continue;
                    } else {
                        self.call_fix_code_bugs(factsheet).await;
                        self.attributes.state = AgentState::UnitTesting;
                        continue;
                    }
                },
                AgentState::UnitTesting => {
                    // setting the agent state to finished to make sure 
                    // we're not running into an infinite loop
                    self.attributes.state = AgentState::Finished;
                },
                _ => {}
            }
        }
        Ok(())
    }
}

/// Writing unit test for the backend developer
#[cfg(test)]
mod test {
    use super::*;
    
    // cargo test tests_writing_backend_code -- --nocapture
    #[tokio::test]
    async fn tests_writing_backend_code() {
        
        let mut agent = AgentBackendDeveloper::new();

        // creating a raw string for the factsheet
        // factsheet code is created by running a test on GPT with
        // cargo test tests_managing_agent -- --nocapture
        // "project_description": "build a website that fetches and tracks fitness progress and includes timezone information from the web",
        /*
EXAMPLE 1 //////////
{
    "project_description": "build a website that only tracks and returns the time of the day",
    "project_scope": {
        "is_crud_required": false,
        "is_user_login_and_logout": false,
        "is_external_urls_required": false
    },
    "external_urls": [],
    "backend_code": null,
    "api_endpoint_schema": null
}
EXAMPLE 2 //////////
{
    "project_description": "build a website that fetches and tracks fitness progress and includes timezone information from the web",
    "project_scope": {
        "is_crud_required": true,
        "is_user_login_and_logout": true,
        "is_external_urls_required": true
    },
    "external_urls": [
        "https://worldtimeapi.org/api/timezone",
        "https://wger.de/api/v2/exerciseinfo/"
    ],
    "backend_code": null,
    "api_endpoint_schema": null
}
        */
        let factsheet_str = r#"
            {
                "project_description": "build a website that fetches and tracks fitness progress and includes timezone information from the web",
                "project_scope": {
                    "is_crud_required": true,
                    "is_user_login_and_logout": true,
                    "is_external_urls_required": true
                },
                "external_urls": [
                    "https://worldtimeapi.org/api/timezone",
                    "https://wger.de/api/v2/exerciseinfo/"
                ],
                "backend_code": null,
                "api_endpoint_schema": null
            }
        "#;

        let mut factsheet: FactSheet = serde_json::from_str(factsheet_str).unwrap();

        agent.execute(&mut factsheet)
            .await
            .expect("Failed to execute Backend Developer Agent");
        
        // dbg!(factsheet);

    }
}
    
