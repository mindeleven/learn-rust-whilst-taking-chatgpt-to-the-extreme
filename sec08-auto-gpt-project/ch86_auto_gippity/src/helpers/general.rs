#![allow(dead_code, unused_imports, unused_variables)]
use std::fmt::format;
use crate::apis::call_request::call_gpt;
use crate::models::general::llm::{self, Message};
use crate::helpers::command_line::PrintCommand;
use serde::de::DeserializeOwned;
use reqwest::Client;
use std::fs;

const CODE_TEMPLATE_PATH: &str = "../../sec07-web-server-template/ch74_web_template/src/code_template.rs";
const EXEC_MAIN_PATH: &str = "../../sec07-web-server-template/ch74_web_template/src/main.rs";
const API_SCHEMA_PATH: &str = "./schemas/api_schema.json";

// appending a string to the and od an ai_function
// extend ai function to encourage specific output
pub fn extend_ai_function(ai_func: fn(&str) -> &'static str, func_input: &str) -> Message {
    
    let ai_function_str = ai_func(func_input);
    // dbg!(ai_function_str);

    // extend the string to encourage only printing the output
    let msg: String = format!("FUNCTION: {}
    INSTRUCTION: You are a function printer. You only print the results of functions.
    Nothing else. No commentary. Here is the input to the function {}.
    Print out what the function will return.",
    ai_function_str, func_input);
    // "Hopefully this is starting to make sense", Shaun just said
    
    // return message
    Message {
        role: "system".to_string(),
        content: msg
    }
}

/// function that prints a message about what the agent actually is doing,
/// (1) extending the ai function, 
/// (2) making the call to chat gpt4,
/// (3) and if it doesn't work making the call again,
/// (4) then, prints a message about what the agent actually is doing
/// goal here is to give a function some input and let it do these three things
// => function that performs call to LLM (or OpenAI, or backend, etc.)
pub async fn ai_task_request(
    // message context is the input to our function == will go into the ai function
    // (the func_input from extend_ai_function)
    msg_context: String,
    // telling the user what agent is doing what
    agent_position: &str, 
    // and what operation are they doing
    agent_operation: &str, 
    // function that we're actually going to pass
    // gets a lifetime with for<'a>
    // (a function passing a function into a function with lifetimes)
    function_pass: for<'a> fn(&'a str) -> &'static str
) -> String {
    
    // extend AI function
    let extended_msg: Message = extend_ai_function(function_pass, &msg_context);

    // print current status
    PrintCommand::AICall.print_agent_message(
        agent_position, 
        agent_operation
    );

    // get LLM response from call_gpt
    // returns Result<String, Box<dyn std::error::Error + Send>>
    let llm_response_res: Result<String, Box<dyn std::error::Error + Send>> 
        = call_gpt(vec![extended_msg.clone()]).await;

    // handle success or try again
    let llm_response = match llm_response_res {
        Ok(llm_res) => llm_res,
        // in case we didn't get a response we call chat gpt again
        Err(_) => call_gpt(vec![extended_msg.clone()])
                      .await
                      .expect("Failed twice to call OpenAI"),
    };

    llm_response
}

// performs call to LLM GPT - Decoded
// LLM is supposed to return a decoded struct (a gerneric type)
pub async fn ai_task_request_decoded<T: DeserializeOwned>(
    msg_context: String,
    agent_position: &str, 
    agent_operation: &str, 
    function_pass: for<'a> fn(&'a str) -> &'static str
) -> T {
    
    let llm_response: String = ai_task_request(
        msg_context, agent_position, agent_operation, function_pass
    ).await;
    
    // decode into struct
    let decoded_response: T = serde_json::from_str(llm_response.as_str())
        .expect("Failed to decode AI response from serde_json");
    
    decoded_response
}

// we need to check that wether url the llm comes up with does work
// example: "I need a webserver that fetches fores price data"
// -> I need to check wether the url I get back actually works
// Check wether request url is valid
pub async fn check_status_code(client: &Client, url: &str) -> Result<u16, reqwest::Error> {
    let response: reqwest::Response = client.get(url).send().await?;
    Ok(response.status().as_u16())
}

// some basic functions that allow us to do a few things
// (1) Get Code Template
// -> we're using webserver template from section 7 as basis for llm to use it as a sceleton
// we need to extract this code as a string and provide it to GPT
pub fn read_code_template_contents() -> String {
    let path = String::from(CODE_TEMPLATE_PATH);
    fs::read_to_string(CODE_TEMPLATE_PATH).expect("Unable to read code template")
}

// (2) Save new Backend Code
// once llm get's into write mode we'll need to safe the code created
pub fn save_backend_code(contents: &String) {
    let path = String::from(EXEC_MAIN_PATH);
    fs::write(path, contents).expect("Failed to write main.rs file");
}

// (3) Save JSON endpoint API schema



#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;
    
    // cargo test tests_extending_ai_function -- --nocapture
    #[test]
    fn tests_extending_ai_function() {
        // let x_str = convert_user_input_to_goal("dummy_variable");
        // dbg!(x_str);
        let extended_msg: Message =
            extend_ai_function(convert_user_input_to_goal, "dummy input");
        dbg!(&extended_msg);
        assert_eq!(extended_msg.role, "system".to_string());
    }
    
    // cargo test tests_ai_task_request -- --nocapture
    #[tokio::test]
    async fn tests_ai_task_request() {
        let ai_func_param = "Build me a webserver for making stock price api requests".to_string();

        let res = ai_task_request(
            ai_func_param,
            "Managing agent", 
            "Defining user requirements", 
            convert_user_input_to_goal
        ).await;
        
        assert!(res.len() > 20);

        dbg!(res);
    }

    // cargo test tests_code_template_path -- --nocapture
    #[test]
    fn tests_code_template_path() {
        use std::fs;
        let code_file = fs::read_to_string(
            CODE_TEMPLATE_PATH
        )
        .expect("Unable to read file");
        dbg!(code_file);

        let exec_file = fs::read_to_string(
            EXEC_MAIN_PATH
        )
        .expect("Unable to read file");
        dbg!(exec_file);

        let schema_file = fs::read_to_string(
            API_SCHEMA_PATH
        )
        .expect("Unable to read file");
        dbg!(schema_file);
    }
    
}