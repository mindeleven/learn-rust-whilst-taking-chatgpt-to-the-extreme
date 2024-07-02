#![allow(dead_code, unused_imports, unused_variables)]

// function returns json formated structure or error from reqwest
async fn my_asnc_call(url: &str) -> Result<serde_json::Value, reqwest::Error> {
    
    let response: serde_json::Value = reqwest::get(url)
        .await?
        .json::<serde_json::Value>()
        .await?;

    Ok(response)
}

#[cfg(test)]
mod test {
    use super::*;
    
    // cargo test tests_error_handling -- --nocapture
    #[tokio::test] 
    async fn tests_error_handling() {

        let api_url = "https://cat-fact.herokuapp.com/facts/";
        // async call to api endpoint
        let my_res: Result<serde_json::Value, reqwest::Error> = my_asnc_call(api_url)
            .await; // needs to be awaited
        // let's see what we got 
        match my_res {
            Ok(value) => { dbg!(value); },
            Err(_) => { panic!("Api call failed!"); },
        };
    
    }

}