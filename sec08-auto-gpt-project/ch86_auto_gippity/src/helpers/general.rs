#![allow(dead_code, unused_imports, unused_variables)]
use std::fmt::format;

use crate::models::general::llm::Message;

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
}