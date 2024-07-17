#![allow(dead_code, unused_imports, unused_variables)]
use crate::models::general::llm::Message;

// appending a string to the and od an ai_function
pub fn extend_ai_function(ai_func: fn(&str) -> &'static str, func_input: &str) -> Message {
    
    let ai_function_str = ai_func(func_input);
    dbg!(ai_function_str);
     
    Message {
        role: "to be replaced later".to_string(),
        content: "to be replaced later".to_string()
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
        extend_ai_function(convert_user_input_to_goal, "dummy input you fool");
    }
}