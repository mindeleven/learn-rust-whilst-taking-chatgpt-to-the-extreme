#![allow(dead_code, unused_imports, unused_variables)]
/// an Arc is essentially the same thing as an Rc
/// it is used primarily when there is multi threading involved
/// an Arc guarantees a reference when used across multiple threads
/// 
/// Mutex provides memory safety when accessing and mutating data across threads
use std::sync::{
    Arc, 
    Mutex,
    Weak
};
use std::thread;

#[derive(Debug)]
struct Owner {
    name: String,
    tools: Mutex<Vec<Weak<Tool>>> // defining Mutex as a vector of weak tools

}

#[derive(Debug)]
struct Tool {
    owner: Arc<Owner>
}

#[cfg(test)]
mod test {
    use std::borrow::BorrowMut;

    use super::*;

    // cargo test tests_mutex -- --nocapture
    #[test] 
    fn tests_mutex() {
        println!("--- ARC BEGIN ---");

        let brad = Arc::from(
            Owner { name: "Brad".to_string(), tools: Mutex::new(vec![]) }
        );
        let pliers = Arc::from(Tool { owner: Arc::clone(&brad) });
        let wrench = Arc::from(Tool { owner: brad.clone() });

        for i in 0..10 {
            // brad moves with each iteration
            // therefor we need to define a new refrence to brad in each iteration
            // otherwise we get a move error because `brad` has type `Arc<Owner>`
            // which does not implement the `Copy` trait
            let brad = Arc::clone(&brad); // defining brad again by cloning brad
            // doing the same for the tools
            let pliers = Arc::clone(&pliers);
            let wrench = Arc::clone(&wrench);

            let child = thread::spawn(move || {
                
                // accessing the Mutex and updating it
                // first generating a Mutex guard to access a Mutex
                let mut guard = brad.tools.lock() // requesting access to the location in memory
                    // lock becomes unlocked once it goes out of scope
                    // returns a result so we need to unwrap
                    .unwrap();
                // now we've our vector and can add tools
                // which need to be downgraded
                guard.push(Arc::downgrade(&pliers));
                guard.push(Arc::downgrade(&wrench));
                
                // first item in Brad's toolkit
                println!("First item in Brad's toolkit: {}", guard[0].upgrade().unwrap().owner.name);


                println!("Pliers owner: {}", pliers.owner.name);

                println!("Thread {} END", i);
            });

            // getting the thread to run
            let res = child.join();
        }

        println!("--- ARC END -----");


    }
}
