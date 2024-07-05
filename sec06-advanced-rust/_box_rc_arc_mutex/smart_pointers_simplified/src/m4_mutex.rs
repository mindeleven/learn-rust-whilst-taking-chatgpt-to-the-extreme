#![allow(dead_code, unused_imports, unused_variables)]
/// an Arc is essentially the same thing as an Rc
/// it is used primarily when there is multi threading involved
/// an Arc guarantees a reference when used across multiple threads
use std::sync::{
    Arc, 
    Weak
};
use std::thread;

#[derive(Debug)]
struct Owner {
    name: String,
    tools: Vec<Weak<Tool>>

}

#[derive(Debug)]
struct Tool {
    owner: Arc<Owner>
}

#[cfg(test)]
mod test {
    use std::borrow::BorrowMut;

    use super::*;

    // cargo test tests_arc -- --nocapture
    #[test] 
    fn tests_arc() {
        println!("--- ARC BEGIN ---");

        let brad = Arc::from(
            Owner { name: "Brad".to_string(), tools: vec![] }
        );

        for i in 0..10 {
            // brad moves with each iteration
            // therefor we need to define a new refrence to brad in each iteration
            // otherwise we get a move error because `brad` has type `Arc<Owner>`
            // which does not implement the `Copy` trait
            let brad = Arc::clone(&brad); // defining brad again by cloning brad
            let child = thread::spawn(move || {
                let pliers = Arc::from(Tool { owner: Arc::clone(&brad) });
                let wrench = Arc::from(Tool { owner: brad.clone() });

                println!("Pliers owner: {}", pliers.owner.name);

                println!("Thread {} END", i);
            });

            // getting the thread to run
            let res = child.join();
        }

        println!("--- ARC END -----");

        dbg!(brad); // so far brad doesn't have any tools

    }
}
