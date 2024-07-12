#![allow(dead_code, unused_imports, unused_variables)]

#[cfg(test)]
mod test {
    use super::*;

    // cargo test tests_declarative_macros -- --nocapture
    #[test] 
    fn tests_declarative_macros() {
        
        // examples of declarative macros
        println!("hello part 1");
        dbg!("tests_declarative_macros");
        let x = vec![1, 2, 3];
        let formated = format!("Hello from 3 with {:?}", x);
        dbg!(formated);

    }
}