#![allow(unused_imports)]

/// purpose of the project:
/// building a web server that does a number of things
/// as a support for a to do app
/// it can create, read, update and delete data 
/// or fetch data from somewhere

use actix_cors::Cors;

use actix_web::{
    http::header, 
    web, 
    App, 
    HttpResponse, 
    HttpServer, 
    Responder
};

use serde::{
    Serialize,
    Deserialize
};

use reqwest::Client as HttpClient;

use async_trait::async_trait;

use std::sync::Mutex;
use std::collections::HashMap;
use std::fs;
use std::io::Write;

/// server is going to represent simple to do app
/// structuring data we're going to store in a json file
/// storing task data
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: u64,
    name: String,
    completed: bool,
}
/// storing user data
#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    id: u64,
    username: String,
    password: String, // easy storing as a string here
}
/// database
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Database {
    tasks: HashMap<u64, Task>,
    users: HashMap<u64, User>,
}

// implementing the database functionality
impl Database {
    // creating new database
    fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            users: HashMap::new(),
        }
    }

    // CRUD DATA RELATED FUNCTIONS
    fn insert(&mut self, task: Task) {
        self.tasks.insert(task.id, task);
    }
    
    fn get(&self, id: &u64) -> Option<&Task> {
        self.tasks.get(id)
    }

    fn get_all(&self) -> Vec<&Task> {
        self.tasks.values().collect()
    }
    
    fn delete(&mut self, id: &u64) {
        self.tasks.remove(id);
    }

    fn update(&mut self, user: User) {
        self.users.insert(user.id, user);
    }

    // USER DATA RELATED FUNCTIONS
    fn insert_user(&mut self, task: Task) {
        self.tasks.insert(task.id, task);
    }

    fn get_user_by_name(&self, username: &str) -> Option<&User> {
        self.users.values().find(|u| u.username == username)
    }

    // DATABASE SAVING
    // converting the hashmap into a using file
    fn save_to_file(&self) -> std::io::Result<()> {
        let data = serde_json::to_string(&self)?;
        let mut file: fs::File = fs::File::create("./data/database.json")?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }
    
    fn load_from_file() -> std::io::Result<Self> { // Self is type Database
        let file_content = fs::read_to_string("./data/database.json")?;
        let db: Database = serde_json::from_str(&file_content)?;
        Ok(db)
    }

}

// we've shared data that users will try to access at the same time
// therefore we implement Mutex
struct AppState {
    // database data will be shared so we wrap it into a Mutex
    db: Mutex<Database>,
}

fn main() {
    println!("Hello, world!");
}
