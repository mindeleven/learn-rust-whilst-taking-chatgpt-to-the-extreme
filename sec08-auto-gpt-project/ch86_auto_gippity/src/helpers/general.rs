#![allow(dead_code, unused_imports, unused_variables)]
use std::fmt::format;
use crate::apis::call_request::call_gpt;
use crate::models::general::llm::{self, Message};
use crate::helpers::command_line::PrintCommand;

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

// function that performs call to LLM/OpenAI
pub async fn ai_task_request(
    // message context is the input to our function == will go into the ai function
    // (the func_input from extend_ai_function)
    msg_context: String,
    agent_position: &str, 
    agent_operation: &str, 
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
}