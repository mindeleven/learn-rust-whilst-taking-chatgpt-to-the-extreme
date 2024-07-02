#![allow(dead_code, unused_imports, unused_variables)]

use std::collections::{
    HashMap,
    HashSet,
};

#[cfg(test)]
mod test {
    use super::*;
    
    // cargo test tests_hashmap -- --nocapture
    #[test] 
    fn tests_hashmap() {
        dbg!("tests_hashmap");
    }

    // cargo test tests_hashset -- --nocapture
    #[test] 
    fn tests_hashset() {
        dbg!("tests_hashset");
    }
}
