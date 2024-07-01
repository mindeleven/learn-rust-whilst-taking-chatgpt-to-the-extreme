#![allow(dead_code, unused_imports, unused_variables)]


#[cfg(test)]
mod test {
    use super::*;
    
    // cargo test tests_calls_asnyc_fn -- --nocapture
    #[tokio::test] 
    async fn tests_calls_asnyc_fn() {
    
        dbg!("tests_calls_asnyc_fn");
    
    }

}