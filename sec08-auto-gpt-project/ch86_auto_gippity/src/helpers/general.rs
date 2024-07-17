// use crate::models::general::llm::Message;

// appending a string to the and od an ai_function
pub fn extend_ai_function(ai_func: fn(&str) -> &'static str, func_input: &str) { // -> Message
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;
    
    // cargo test tests_extending_ai_function -- --nocapture
    #[test]
    fn tests_extending_ai_function() {
        dbg!("tests_extending_ai_function");
    }
}