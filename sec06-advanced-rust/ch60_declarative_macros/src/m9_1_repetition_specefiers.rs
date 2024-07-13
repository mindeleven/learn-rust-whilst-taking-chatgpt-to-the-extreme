#![allow(dead_code, unused_imports, unused_variables)]

// MACRO CAPTURES accompanying the course taken from
// https://github.com/coderaidershaun/rust_course_playaround/blob/main/src/m9_decl_macros.rs

// REPITITION SPECIFIER

// * - match zero or more repititions
// + - match one or more repititions
// ? - Match zero or one repetition

/*
Note: To enable a macro to be used in other files if the crate containing your macro is imported, 
add this above your macro:

#[macro_export]

macro_rules! my_vec { ... }
*/

macro_rules! my_vec {
    // 1. write captures: we need to take in an expr
    // wrapping the expression and makinf it an expression itself
    // second param: how many are we going to have -> REPITITION SPECIFIER
    // + - match one or more repititions
    ($($x: expr), +) => {
        {
            let mut temp_vec = Vec::new();
            // opening the repetition block
            $(
                // extracting $x
                temp_vec.push($x);
            )+ // number of repetitions at the end
            // returning vector
            temp_vec 
        }
    }
}


#[cfg(test)]
mod test {
    
    // creating a declarative macro
    // creating a declarative macro with repititions
    // From Udemy: Declarative Macros - Note on Module Exporting
    // Note: To enable a macro to be used in other files if the crate containing 
    // your macro is imported, add this above your macro:
    /*
    #[macro_export]

    macro_rules! my_vec { ... }
    */

    // cargo test tests_repetition_specifiers -- --nocapture
    #[test] 
    fn tests_repetition_specifiers() {
        
        // example of declarative macro with repetition specifiers
        println!("-----------------------------------------------------------");
        println!("// example of declarative macros with repetition specifiers");
        println!("-----------------------------------------------------------");
        let mut y: Vec<i32> = my_vec!(1);
        y.push(34);
        dbg!(y);

    }
}