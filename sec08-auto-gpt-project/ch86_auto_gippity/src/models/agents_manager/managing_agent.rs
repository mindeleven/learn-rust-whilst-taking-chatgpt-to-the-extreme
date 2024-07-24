// managing agent will be responsible for managing agents with a special given trait
use crate::models::agent_basic::basic_agent::{ AgentState, BasicAgent };
use crate::models::agents::agent_traits::FactSheet;

#[derive(Debug)]
pub struct ManagingAgent {
    attributes: BasicAgent,
    factsheet: FactSheet,
    agent: Vec<String> // vector of agents that have a particular trait
}