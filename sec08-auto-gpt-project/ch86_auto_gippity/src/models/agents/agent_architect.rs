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
#[derive(Debug)]
pub struct AgentSolutionArchitect {
    pub attributes: BasicAgent,
}

impl AgentSolutionArchitect {
    pub fn new() -> Self {
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
    
    // determine if external urls are needed
    // makes update to factsheet, doesn't return anything
    async fn call_determine_external_urls(
        &mut self, 
        factsheet: &mut FactSheet,
        msg_context: String
    ) {

        let ai_response: Vec<String> = ai_task_request_decoded::<Vec<String>>(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_site_urls),
            print_site_urls
        ).await;

        factsheet.external_urls = Some(ai_response);
        self.attributes.state = AgentState::UnitTesting;

    }

}

// implementing special functions for the solutions architect
#[async_trait]
impl SpecialFunctions for AgentSolutionArchitect {
    fn get_attributes_from_agent(&self) -> &BasicAgent {
        // returning all the information about this agent
        &self.attributes
    }
    
    async fn execute(
        &mut self, 
        factsheet: &mut FactSheet
    ) -> Result<(), Box<dyn std::error::Error>> {

        // WARNING - be careful of infinite loops
        while self.attributes.state != AgentState::Finished {

            match self.attributes.state {
                
                AgentState::Discovery => {

                    let project_scope 
                        = self.call_project_scope(factsheet).await;

                    // confirm if external urls
                    if project_scope.is_external_urls_required {
                        self.call_determine_external_urls(
                            factsheet, factsheet.project_description.clone()
                        ).await;
                        self.attributes.state = AgentState::UnitTesting;
                    }
                },
                AgentState::UnitTesting => {
                    // unit testing for the solutions architect is about testing
                    // that the urls from the Discovery phase are actually working
                    let mut exclude_urls: Vec<String> = vec![];

                    let client = Client::builder()
                        .timeout(Duration::from_secs(5))
                        .build()
                        .unwrap();

                    // find faulty urls
                    let urls: &Vec<String> = factsheet
                        .external_urls.as_ref().expect("No URL object on factsheet");

                    // finding the faulty urls
                    for url in urls {
                        let endpoint_str = format!("Testing URL endpoint {}", url);
                        PrintCommand::UnitTest.print_agent_message(
                            self.attributes.position.as_str(),
                            endpoint_str.as_str()
                        );

                        // perform URL test
                        match check_status_code(&client, url).await {
                            Ok(status_code) => {
                                if status_code != 200 {
                                    exclude_urls.push(url.clone())
                                }
                            },
                            Err(e) => {
                                println!("Error checking {}: {}", url, e);
                            }

                        }                        
                    }

                    // exclude any faulty urls
                    if exclude_urls.len() > 0 {
                        let new_urls: Vec<String> = factsheet
                            .external_urls
                            .as_ref()
                            .unwrap()
                            .iter()
                            .filter(|url| !exclude_urls.contains(&url))
                            .cloned()
                            .collect();

                        factsheet.external_urls = Some(new_urls);
                    }

                    // confirm done
                    self.attributes.state = AgentState::Finished;

                },
                // default to finished state
                _ => {
                    self.attributes.state = AgentState::Finished;
                }
            }
        }

        Ok(())
    }

}

/// Writing unit test for the solutions architect
#[cfg(test)]
mod test {
    use super::*;
    
    // cargo test tests_solution_architect -- --nocapture
    #[tokio::test]
    async fn tests_solution_architect() {
        let mut agent = AgentSolutionArchitect::new();

        // the factsheet comes from the managing agent but as we don't have a 
        // managing agent here, a dummy fact sheet needs to be created
        let mut factsheet: FactSheet = FactSheet {
            project_description: "Build a full stack website with user login and logout that shows latest Forex prices".to_string(),
            project_scope: None,
            external_urls: None,
            backend_code: None,
            api_endpoint_schema: None,
        };

        agent.execute(&mut factsheet).await.expect("Unable to execute Solution Architect Agent!");
        assert!(factsheet.project_scope != None);
        assert!(factsheet.external_urls.is_some());

        dbg!(factsheet);
    }
}