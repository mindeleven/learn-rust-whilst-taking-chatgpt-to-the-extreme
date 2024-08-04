use actix_cors::Cors;

use actix_web::{http::header, web, App, HttpServer, Responder, HttpResponse};

use serde::{Deserialize, Serialize};

use reqwest::Client as HttpClient;

use async_trait::async_trait;

use std::sync::Mutex;
use std::collections::HashMap;
use std::fs;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct FitnessData {
    id: u64,
    user_id: u64,
    activity: String,
    duration: u64,
    timestamp: String,
    timezone: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    id: u64,
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Database {
    fitness_data: HashMap<u64, FitnessData>,
    users: HashMap<u64, User>,
}

impl Database {
    fn new() -> Self {
        Self {
            fitness_data: HashMap::new(),
            users: HashMap::new(),
        }
    }

    // CRUD DATA
    fn insert_fitness_data(&mut self, data: FitnessData) {
        self.fitness_data.insert(data.id, data);
    }

    fn get_fitness_data(&self, id: &u64) -> Option<&FitnessData> {
        self.fitness_data.get(id)
    }

    fn get_all_fitness_data(&self) -> Vec<&FitnessData> {
        self.fitness_data.values().collect()
    }

    fn delete_fitness_data(&mut self, id: &u64) {
        self.fitness_data.remove(id);
    }

    fn update_fitness_data(&mut self, data: FitnessData) {
        self.fitness_data.insert(data.id, data);
    }

    // USER DATA RELATED FUNCTIONS
    fn insert_user(&mut self, user: User) {
        self.users.insert(user.id, user);
    }

    fn get_user_by_name(&self, username: &str) -> Option<&User> {
        self.users.values().find(|u| u.username == username)
    }

    // DATABASE SAVING
    fn save_to_file(&self) -> std::io::Result<()> {
        let data: String = serde_json::to_string(&self)?;
        let mut file: fs::File = fs::File::create("database.json")?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    fn load_from_file() -> std::io::Result<Self> {
        let file_content: String = fs::read_to_string("database.json")?;
        let db: Database = serde_json::from_str(&file_content)?;
        Ok(db)
    }
}

struct AppState {
    db: Mutex<Database>,
    http_client: HttpClient,
}

async fn create_fitness_data(app_state: web::Data<AppState>, data: web::Json<FitnessData>) -> impl Responder {
    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    db.insert_fitness_data(data.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}

async fn read_fitness_data(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    match db.get_fitness_data(&id.into_inner()) {
        Some(data) => HttpResponse::Ok().json(data),
        None => HttpResponse::NotFound().finish(),
    }
}

async fn read_all_fitness_data(app_state: web::Data<AppState>) -> impl Responder {
    let db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    let data = db.get_all_fitness_data();
    HttpResponse::Ok().json(data)
}

async fn update_fitness_data(app_state: web::Data<AppState>, data: web::Json<FitnessData>) -> impl Responder {
    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    db.update_fitness_data(data.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}

async fn delete_fitness_data(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    db.delete_fitness_data(&id.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}

async fn register(app_state: web::Data<AppState>, user: web::Json<User>) -> impl Responder {
    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    db.insert_user(user.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}

async fn login(app_state: web::Data<AppState>, user: web::Json<User>) -> impl Responder {
    let db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    match db.get_user_by_name(&user.username) {
        Some(stored_user) if stored_user.password == user.password => {
            HttpResponse::Ok().body("Logged in!")
        }
        _ => HttpResponse::BadRequest().body("Invalid username or password"),
    }
}

async fn fetch_timezone(http_client: &HttpClient, timezone: &str) -> Result<String, reqwest::Error> {
    let url = format!("https://worldtimeapi.org/api/timezone/{}", timezone);
    let response = http_client.get(&url).send().await?;
    let json: serde_json::Value = response.json().await?;
    Ok(json["datetime"].as_str().unwrap_or_default().to_string())
}

async fn fetch_exercise_info(http_client: &HttpClient, exercise_id: u64) -> Result<serde_json::Value, reqwest::Error> {
    let url = format!("https://wger.de/api/v2/exerciseinfo/{}", exercise_id);
    let response = http_client.get(&url).send().await?;
    let json: serde_json::Value = response.json().await?;
    Ok(json)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db: Database = match Database::load_from_file() {
        Ok(db) => db,
        Err(_) => Database::new(),
    };

    let data: web::Data<AppState> = web::Data::new(AppState {
        db: Mutex::new(db),
        http_client: HttpClient::new(),
    });

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::permissive()
                    .allowed_origin_fn(|origin, _req_head| {
                        origin.as_bytes().starts_with(b"http://localhost") || origin == "null"
                    })
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .app_data(data.clone())
            .route("/fitness_data", web::post().to(create_fitness_data))
            .route("/fitness_data", web::get().to(read_all_fitness_data))
            .route("/fitness_data", web::put().to(update_fitness_data))
            .route("/fitness_data/{id}", web::get().to(read_fitness_data))
            .route("/fitness_data/{id}", web::delete().to(delete_fitness_data))
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}