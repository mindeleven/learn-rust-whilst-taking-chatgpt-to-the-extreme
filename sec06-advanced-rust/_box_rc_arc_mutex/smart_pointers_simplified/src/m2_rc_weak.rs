#![allow(unused_imports)]
/// and now: the Weak version of Rc<>
/// Weak guarantees the reference but it does not guarantee the value
/// example same as m1_rc.rs
/// BUT if we want the list of tools to be owned by brad we need to create a list of Weak tools
/// we don't use Rc because Rc can lead to a memory leak
/// AND we define the Tools vector as a RefCell so we can borrow a mutable version of it
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
    // if we want the list of tools to be owned by brad
    // we need to create a vector of weak tools
    // Rc could lead to a memory leak so we're using Weak
    // tools: Vec<Weak<Tool>>,
    // defining the tools vector as a ref cel so we can borrow a mutable version of it
    tools: RefCell<Vec<Weak<Tool>>>,

}

#[derive(Debug)]
struct Tool {
    // to create a reference around brad we define owner as type Rc<>
    owner: Rc<Owner>
}

#[cfg(test)]
mod test {
    use std::borrow::BorrowMut;

    use super::*;

    // cargo test tests_rc_weak -- --nocapture
    #[test] 
    fn tests_rc_weak() {
        
        let brad = Rc::from(
            Owner { name: "Brad".to_string(), tools: RefCell::new(vec![]) }
        );
        // creating weak version of tools with Rc
        let pliers = Rc::from(Tool { owner: Rc::clone(&brad) });
        let wrench = Rc::from(Tool { owner: brad.clone() });

        // now, borrowing a mutable version of brad tools
        // tools still need to be downgraded from a strong reference to a weak reference
        brad.tools.borrow_mut().push(Rc::downgrade(&pliers));
        brad.tools.borrow_mut().push(Rc::downgrade(&wrench));

        dbg!(brad.clone());

        // printing a tool out by upgrading the weak references to Rc
        // to have actual access to the value
        println!("Pliers owner: {}", pliers.owner.name);
        println!("Brad plier's owner: {:?}", brad.tools.borrow()[0].upgrade().unwrap().owner.name);
    }
}
