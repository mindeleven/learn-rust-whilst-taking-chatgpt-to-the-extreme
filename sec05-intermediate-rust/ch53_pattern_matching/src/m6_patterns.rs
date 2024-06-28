#![allow(dead_code, unused_imports, unused_variables)]


#[cfg(test)]
mod test {
    use std::num::ParseIntError;

    use super::*;
    
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

}
