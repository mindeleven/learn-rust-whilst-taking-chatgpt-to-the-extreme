// managing agent will be responsible for managing agents with a special given trait
use crate::models::agent_basic::basic_agent::{ AgentState, BasicAgent };
use crate::models::agents::agent_traits::{ FactSheet, SpecialFunctions };

use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;
use crate::helpers::general::ai_task_request;
use crate::models::agents::agent_architect::AgentSolutionArchitect;
use crate::models::general::llm::Message;

#[derive(Debug)]
pub struct ManagingAgent {
    // our attributes will be of basic agent and all agents will have these attributes
    attributes: BasicAgent,
    factsheet: FactSheet,
    // giving agents the SpecialFunctions trait which will allow them 
    // to use the functions defined there
    // each agent will have the ability to see the factsheet and update it accordingly
    agents: Vec<Box<dyn SpecialFunctions>> // vector of agents that have a particular trait
}

impl ManagingAgent {
    pub async fn new(usr_req: String) -> Result<Self, Box<dyn std::error::Error>> {
        let position = "Project Manager".to_string();

        let attributes = BasicAgent {
            objective: "Manage agents who are building an excellent website for the user".to_string(),
            position: position.clone(),
            state: AgentState::Discovery,
            memory: vec![]
        };

        let project_description: String = ai_task_request(
            usr_req,
            &position,
            get_function_string!(convert_user_input_to_goal),
            convert_user_input_to_goal
        ).await;

        let agents: Vec<Box<dyn SpecialFunctions>> = vec![];

        let mut factsheet: FactSheet = FactSheet {
            project_description,
            project_scope: None,
            external_urls: None,
            backend_code: None,
            api_endpoint_schema: None,
        };

        Ok(Self{
            attributes,
            factsheet,
            agents
        })
    }

    pub fn add_agent(&mut self, agent: Box<dyn SpecialFunctions>) {
        self.agents.push(agent);
    }

    fn create_agents(&mut self) {
        self.add_agent(Box::new(AgentSolutionArchitect::new()));
        // ! TODO add backend agent
    }

    pub async fn execute_project(&mut self) {
        self.create_agents();

        for agent in &mut self.agents {
            let agent_res: Result<(), Box<dyn std::error::Error>> 
                = agent.execute(&mut self.factsheet).await;

            let agent_info = agent.get_attributes_from_agent();
            dbg!(agent_info);
        }
    }

}


/// Writing unit test for the managing agent
#[cfg(test)]
mod test {
    use super::*;
    
    // cargo test tests_managing_architect -- --nocapture
    #[tokio::test]
    async fn tests_managing_architect() {
        let usr_request: &str 
            = "need a full stack app that fetches and tracks my fitness progress. Needs to include timezone info from the web.";
        
        let mut managing_agent: ManagingAgent = 
            ManagingAgent::new(usr_request.to_string())
                .await
                .expect("Error creating Managing Agent");

        managing_agent.execute_project().await;

        dbg!(managing_agent.factsheet);
    }
}