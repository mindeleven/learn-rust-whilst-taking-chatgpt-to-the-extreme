/// same as m10_proc_macros.rs, just with ai_functions from crates.io
/// source at crates.io: 
/// @ https://crates.io/crates/ai_functions
/// docs @ docs.rs/ai_functions/0.1.1
#[cfg(test)]
#[allow(dead_code, unused_imports, unused_variables)]
mod test {
    use ai_functions::ai_function;

    const OUTPUT: &str = "";

    #[ai_function]
    fn some_function_for_ai_llm(_some_param: &str) {
        /// this is an awesome function
        /// we shall give it to an AI to guess and output
        /// in a structured manner
        println!("{}", OUTPUT);
    }

    #[test]
    // cargo test tests_ai_function -- --nocapture
    fn tests_ai_function() {

        let x = some_function_for_ai_llm("some_param");
        dbg!(x);

    }
}
