#[cfg(test)]
#[allow(dead_code, unused_imports, unused_variables)]
mod test {
    use my_proc_macro::function_to_string;

    const OUTPUT: &str = "";

    #[function_to_string]
    fn some_function_for_ai_llm(_some_param: &str) {
        /// this is an awesome function
        /// we shall give it to an AI to guess and output
        /// in a structured manner
        println!("{}", OUTPUT);
    }

    #[test]
    // cargo test tests_proc_macro -- --nocapture
    fn tests_proc_macro() {

        let x = some_function_for_ai_llm("some_param");
        dbg!(x);

    }
}
