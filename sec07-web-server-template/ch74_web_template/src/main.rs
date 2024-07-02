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

fn main() {
    println!("Hello, world!");
}
