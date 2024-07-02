#![allow(dead_code, unused_imports, unused_variables)]

use std::io::{
    Error,
    ErrorKind
};

// function returns json formated structure or error from reqwest
async fn my_asnc_call(url: &str) -> Result<serde_json::Value, Error> {
    
    /* 
    let response: serde_json::Value = reqwest::get(url)
        .await?
        .json::<serde_json::Value>()
        .await?; 
    */
    let response = reqwest::get(url)
       .await
       .map_err(|_| {
            Error::new(
              ErrorKind::Other,
              "Could not retrieve response"
            )
       })?; // question mark here is going to unwrap or return an error in the io::Error format

    let json_response = response
        .json::<serde_json::Value>()
        .await
        .map_err(|_| {
            Error::new(
            ErrorKind::Other,
            "Could not decode to JSON"
            )
    })?;

    Ok(json_response)
}

#[cfg(test)]
mod test {
    use super::*;
    
    // cargo test tests_error_handling -- --nocapture
    #[tokio::test] 
    async fn tests_error_handling() {

        let api_url = "https://cat-fact.herokuapp.com/facts/";
        // async call to api endpoint
        let my_res: Result<serde_json::Value, std::io::Error> = my_asnc_call(api_url)
            .await; // needs to be awaited
        // let's see what we got 
        match my_res {
            Ok(value) => { dbg!(value); },
            Err(_) => { panic!("Api call failed!"); },
        };
    
    }

}