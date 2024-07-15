extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input,
    DeriveInput
};

// modifying macro following example 
// @ https://github.com/dtolnay/syn/tree/master/examples/heapsize
// HelloWorld is defined as a trait in hello_world sub project

// writing a derive macro function
#[proc_macro_derive(HelloWorld)]
// if this macro gets wrappes around a struct,
// the struct has a bunch of tokens it takes in
// the tokens we got we parse into a syntax tree
pub fn helloworld_object_derive(input: TokenStream) -> TokenStream {
    
    // parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    // geting the identifier out of the input
    let name = input.ident;
    // this will be used in the quasi-quotation below as the name 
    // of the type to implement
    //
    // now: generate the code to paste into the users program
    // with the quote! macro

    let expand = quote! {
        // allows us to take this implementation of HelloWorld and apply it to the name
        impl hello_world::HelloWorld for #name {
            // adds the following function to our struct
            fn hello_world(&self) {
                println!("Printing from the implemented trait");
            }
        }
    };

    // hand the output tokens back to the compiler
    TokenStream::from(expand)
    
}