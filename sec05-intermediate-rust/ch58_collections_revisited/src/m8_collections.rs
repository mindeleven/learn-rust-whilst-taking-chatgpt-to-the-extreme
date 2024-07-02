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
        // hashmaps store key-value pairs
        let mut results_hm: HashMap<&str, u32> = HashMap::new();
        let person_1 = "Alice";
        let person_2 = "Bob";
        results_hm.insert(&person_1, 55);
        results_hm.insert(&person_2, 52);
        // hashmap.get returns an option
        let test_result = results_hm.get(person_1);
        dbg!(test_result);
    }

    // cargo test tests_hashset -- --nocapture
    #[test] 
    fn tests_hashset() {
        // using a hashset to store values in general
        dbg!("tests_hashset");
    }
}
