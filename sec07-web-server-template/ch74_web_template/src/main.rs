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


fn main() {
    println!("Hello, world!");
}
