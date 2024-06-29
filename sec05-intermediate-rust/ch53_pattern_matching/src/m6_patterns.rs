#![allow(dead_code, unused_imports, unused_variables)]

use std::thread::yield_now;

enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String),
}

fn process_msg(msg: Message) {
    match msg {
        Message::Quit => {
            println!("I quit");
        },
        Message::ChangeColor(red, blue, green) => {
            println!("Red: {}, Blue: {}, Green: {}", red, blue, green);
        },
        Message::Move {x, y: new_name} => {
            println!("x is {},  y as new_name is {}", x, new_name);
        },
        Message::Write(text) => {
            println!("{}", text);
        }  
    };
}

#[cfg(test)]
mod test {
    use std::num::ParseIntError;

    use super::*;
    
    // cargo test tests_large_enum -- --nocapture
    #[test] 
    fn tests_large_enum() {
        let my_quit = Message::Quit;
        let my_color = Message::ChangeColor(10, 20, 255);
        let my_move = Message::Move { x: 10, y: 200 };
        let my_writing = Message::Write("Once upon a time".to_string());
        process_msg(my_quit);
        process_msg(my_color);
        process_msg(my_move);
        process_msg(my_writing);
    }

    // cargo test tests_match_literals -- --nocapture
    #[test] 
    fn tests_match_literals() {

        let number: i32 = 20;

        let res: &str = match number {
            1 => "number is one",
            2 | 3 | 9 | 15 | 20 => "number is 2, 3, 9, 15 or 20",
            _ => "it's still another number"
        };

        dbg!(res);
    
    }
    
    // cargo test tests_match_option -- --nocapture
    #[test] 
    fn tests_match_option() {
        
        let some_num: Option<i32> = Some(10);
        
        let res: i32 = match some_num {
            Some(i) =>  i,
            None => {
                panic!("There was a problem");
            },
        };
        dbg!(res);

        let prob_none: Option<i32> = None;
        let prob_none: Option<i32> = Some(66); // comment this line out to panic

        let res_prob: i32 = match prob_none {
            Some(i) =>  i,
            None => {
                panic!("There was a problem"); // panicked at src/m6_patterns.rs:44:17:
            },
        };
        dbg!(res_prob);

    }

    // cargo test tests_if_let -- --nocapture
    #[test] 
    fn tests_if_let() {
        
        let some_num: Option<i32> = Some(10);

        // if let
        // if some_num is of type Some()
        // then evaluate the value of the Some
        let res = if let Some(i) = some_num {
            i // return i
        } else {
            panic!("There was a problem"); // panic!!!!!
        };

        dbg!(res);

    }

    // cargo test tests_match_result -- --nocapture
    #[test] 
    fn tests_match_result() {
        
        let some_res: Result<i32, &str> = Ok(10);
        let some_err: Result<i32, &str> = Err("There was an error");


        let res = match some_res {
            Ok(val) =>  val,
            Err(e) => {
                panic!("There was an error: {}", e);
            },
        };
        dbg!(res);

        let r = if let Ok(r) = some_res {
            r
        } else {
            panic!("There was a problem"); // panic!!!!!
        };

        dbg!(r);

    }

}
