/// GPT Chat Completions API
/// https://platform.openai.com/docs/guides/text-generation/chat-completions-api
#[macro_export]
macro_rules! get_function_string {
    ($func: ident) => {{
        stringify!($func)
    }};
}

#[macro_use]
mod ai_functions;
mod apis;
mod helpers;
mod models;


use helpers::command_line::get_user_response;

fn main() {
    let user_req = get_user_response( // helpers::command_line::get_user_response
        "What webserver are we building today?"
    );
    dbg!(user_req);

    // next thing to do: pass the request to our agent
}
