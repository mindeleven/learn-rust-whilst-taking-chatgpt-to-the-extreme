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
    
    

}
