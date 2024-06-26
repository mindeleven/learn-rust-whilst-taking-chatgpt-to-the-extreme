#![allow(dead_code, unused_imports)]

#[derive(Debug)]
struct ThisIsMyFirstStruct;

#[cfg(test)]
mod test {
    use super::*;
    
    // cargo test tests_structs -- --nocapture
    #[test] 
    fn tests_structs() {

        dbg!("test structs");

    }

}




