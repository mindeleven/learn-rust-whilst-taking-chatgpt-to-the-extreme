/// GPT Chat Completions API
/// https://platform.openai.com/docs/guides/text-generation/chat-completions-api

mod ai_functions;
mod apis;
mod helpers;
mod models;

use helpers::command_line::get_user_response;
use std::fs;

use apis::call_request::api_key_test;

fn main() {
    /* 
    let api_key = fs::read_to_string(
        "../../../../../../tmp/chat_gpt/gippity_key_1.txt"
    ).expect("Unable to read file");

    let org_id = fs::read_to_string(
        "../../../../../../tmp/chat_gpt/org_id.txt"
    ).expect("Unable to read file");

    dbg!(api_key);
    dbg!(org_id);
    */
    
    api_key_test();

    let user_req = get_user_response(
        "What webserver are we building today?"
    );
    dbg!(user_req);

    // next thing to do: pass the request to our agent
}
