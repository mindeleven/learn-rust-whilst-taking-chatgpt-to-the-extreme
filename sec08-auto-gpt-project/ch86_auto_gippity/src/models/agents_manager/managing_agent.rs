// managing agent will be responsible for managing agents with a special given trait
use crate::models::agent_basic::basic_agent::{ AgentState, BasicAgent };
use crate::models::agents::agent_traits::{ FactSheet, SpecialFunctions };

#[derive(Debug)]
pub struct ManagingAgent {
    attributes: BasicAgent,
    factsheet: FactSheet,
    // giving agents the SpecialFunctions trait which will allow them 
    // to use the functions defined there
    agent: Vec<Box<dyn SpecialFunctions>> // vector of agents that have a particular trait
}