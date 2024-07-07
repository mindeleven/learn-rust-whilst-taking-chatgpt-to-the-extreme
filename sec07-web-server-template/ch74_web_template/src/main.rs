#![allow(unused_imports)]

/// purpose of the project:
/// building a web server that does a number of things
/// as a support for a to do app
/// it can create, read, update and delete data 
/// or fetch data from somewhere

use actix_cors::Cors;

use actix_web::web::post;
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
struct AppState { // state of the application
    // database data will be shared so we wrap it into a Mutex
    db: Mutex<Database>,
}

// call to webserver
// returns responder from actix web
// passing in the app state and data (the dask) in Json format
// the type returned will implement the Responder trait
async fn create_task(app_state: web::Data<AppState>, task: web::Json<Task>) -> impl Responder {
    // db is wrapped in Mutex so we need to lock it
    let mut db = app_state.db.lock().unwrap();
    // creating a task
    // into_inner is extracting the task out of what it's wrapped into
    db.insert(task.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // load the database or create a new one
    let db = match Database::load_from_file() {
        Ok(db) => db,
        Err(_) => Database::new()
    }; // what we get back here is a Database

    let data = web::Data::new(AppState {
        // wrapping our Database into a Mutex so we can share it on multiple threads
        db: Mutex::new(db), // wrapping our Database into a Mutex, see definition above
    });
    
    // creating new server
    HttpServer::new(move || {
        // creating new App
        App::new()
            // cross origin resource sharing
            // == making calls to our web server from any port or domain
            // handling calls with wrap()
            .wrap(
                Cors::permissive()
                    .allowed_origin_fn(|origin, _req_head| {
                        origin.as_bytes().starts_with(b"http://localhost") 
                            || origin == "null"

                    })
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![header::AUTHORIZATION, header:: ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600)
            )
            .app_data(data.clone()) // is not cloning as a deep copy bc web::Data is a smart pointer
            .route("/task", post().to(create_task)) // to means what function to run
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
        
}
