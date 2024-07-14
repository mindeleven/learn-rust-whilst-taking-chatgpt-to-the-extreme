// example of a derive macro that just appends a function answer
// short example taken from the Rust book
// @ https://doc.rust-lang.org/reference/procedural-macros.html
extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_derive(AnswerFn)]
pub fn derive_answer_fn(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}