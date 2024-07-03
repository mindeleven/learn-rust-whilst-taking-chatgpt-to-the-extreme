#![allow(unused_imports)]
/// let's define Rc<>
/// Rc stands for reference counter but it's more like a "reference copier"
/// an Rc creates a new reference to an existing piece of data
/// you can have two refrences that are owned separately and point both at the same structure
use std::{
    rc::{
        Rc, // the strong version
        Weak // the Weak version of Rc
    }, 
    cell::RefCell
};

/// example: one owner can have many tools
/// a tool can have only one owner
#[derive(Debug)]
struct Owner {
    name: String,
    tools: Vec<Tool>
}

#[derive(Debug)]
struct Tool {
    // to create a reference around brad we define owner as type Rc<>
    owner: Rc<Owner>
}

#[cfg(test)]
mod test {
    use super::*;

    // cargo test tests_reference_counter -- --nocapture
    #[test] 
    fn tests_reference_counter() {
        
        let brad = Rc::from(Owner { name: "Brad".to_string(), tools: vec![] });
        let pliers = Tool { owner: Rc::clone(&brad) };
        let wrench = Tool { owner: brad.clone() };

        dbg!(brad.clone());
        dbg!(pliers);
        dbg!(wrench);
    }
}
