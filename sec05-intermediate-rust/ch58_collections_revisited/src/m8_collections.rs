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
        // dbg!(test_result); // returns Some()
        dbg!(test_result.unwrap()); // unwraps Some()

        // 
        if results_hm.contains_key("Alice") {
            dbg!("Alice was here!");
        }
    }

    // cargo test tests_hashset -- --nocapture
    #[test] 
    fn tests_hashset() {
        // using a hashset to store values in general
        // storing items in a basic set
        let mut names_hs: HashSet<&str> = HashSet::new();
        names_hs.insert("Bob");
        names_hs.insert("Alice");
        names_hs.insert("Jane");

        if names_hs.contains("Bob") {
            dbg!("Bob is here!");
        }
    }
   
}
