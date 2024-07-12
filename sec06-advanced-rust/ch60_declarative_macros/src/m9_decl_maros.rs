#![allow(dead_code, unused_imports, unused_variables)]

// MACRO CAPTURES accompanying the course taken from
// https://github.com/coderaidershaun/rust_course_playaround/blob/main/src/m9_decl_macros.rs

/* expr
    matches to a valid rust expression
    "hello".to_string(), vec![1, 2, 3], 1 + 2, 1
*/

/* stmt
    matches to a rust statement
    let x = 5, x.push(1), return Some(x)
*/

/* ident
    matches to a rust identifier
    variable name, function name, module name
*/

/* ty
    matches to a rust type
    i32, Vec<String>, Option<T>
*/

/* path
    matches to a rust path
    std::collections::HashMap
*/

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


#[cfg(test)]
mod test {
    
    // creating a declarative macro
    macro_rules! mad_skills {
        // taking some kind of input and returning an operational function
        // what we expect to receive inside () is the capture
        // see list above for possible macro captures that can be matched here
        // `expr` example
        ($x: expr) => {
            // printing out what the expression is
            format!("You sent an expression: {}", $x) // format returns a string
        };
    }

    macro_rules! mad_type_skills {
        ($x: ty) => {
            match stringify!($x) {
                "i32" => "You sent an i32 type".to_string(),
                _ => "You sent something else".to_string(),
            }
        };
    }

    // cargo test tests_declarative_macros -- --nocapture
    #[test] 
    fn tests_declarative_macros() {
        
        // examples of declarative macros

        println!("---------------------------------");
        println!("// examples of declarative macros");
        println!("---------------------------------");
        println!("hello part 1");
        dbg!("tests_declarative_macros");
        let x = vec![1, 2, 3];
        let formated = format!("Hello from 3 with {:?}", x);
        dbg!(formated);

        println!("---------------------------------");
        println!("// creating a declarative macro");
        println!("---------------------------------");
        // using mad_skills with an expression
        let some_skill = mad_skills!(1 + 9);
        dbg!(some_skill);

        // using mad_type_skills with a type
        let some_ty_skill = mad_type_skills!(i32); // passing in a type
        dbg!(some_ty_skill);
        let some_ty_skill = mad_type_skills!(String); // passing in a type
        dbg!(some_ty_skill);

    }
}