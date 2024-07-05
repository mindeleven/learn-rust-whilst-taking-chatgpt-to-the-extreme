#![allow(dead_code, unused_imports, unused_variables)]

#[cfg(test)]
mod test {
    use std::{
        fs::{
           File, OpenOptions
        },
        io::prelude::*,
        sync::{
            Arc,
            Mutex, MutexGuard
        },
        thread::{
            JoinHandle,
            spawn
        },
    };

    // cargo test tests_arc_mutex -- --nocapture
    #[test] 
    fn tests_arc_mutex() {

        // example: programm that has multiple threats that need to access a file concurrently
        // -> Mutex allows to create a variable that needs to be locked before it can be accessed
        // open file and lock it with Mutex
        // what we need too is counting how many references of this file have been opened
        // -> Atomic Reference Counter Arc which is designed to be thread safe
        let file_mutex = Arc::new(Mutex::new(OpenOptions::new()
            .write(true) // write to file if exists
            .create(true) // create file if it doesn't exist
            .append(true) // append to the data rather than replacing it
            .open("./data/increments.txt") // open file
            .unwrap()));  // open file and lock it with Mutex, wrap in Arc
        
        // creating vector to join the threats
        let mut handles: Vec<JoinHandle<()>> = vec![];

        // creating a multiple thread
        for i in 0..13 {
            // creating a reference to file_mutex for each thread
            let file_mutex = Arc::clone(&file_mutex);
            // spawning a new thread
            // moving ownershop into the scope within the closure with `move``
            let handle = spawn(move || {
                // performing operation on the file
                // accessing the file and locking it
                let mut file: MutexGuard<File> = file_mutex.lock().unwrap();
                // writing lines to file
                writeln!(file, "{}", i).unwrap();

            });
            // adding handles to vector
            handles.push(handle);
        }
        
        // telling program to wait for all threads to finish
        for handle in handles {
            // waiting for all spawn threads to finish
            handle.join().unwrap();
        }

    }
}
        
