use actix_cors::Cors;

use actix_web::{http::header, web, App, HttpServer, Responder, HttpResponse};

use serde::{Deserialize, Serialize};

use reqwest::Client as HttpClient;

// use async_trait::async_trait;

use std::sync::Mutex;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ForexPair {
    pair: String,
    price: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Database {
    forex_pairs: HashMap<String, ForexPair>,
}

impl Database {
    fn new() -> Self {
        Self {
            forex_pairs: HashMap::new(),
        }
    }

    fn insert(&mut self, forex_pair: ForexPair) {
        self.forex_pairs.insert(forex_pair.pair.clone(), forex_pair);
    }

    fn get(&self, pair: &str) -> Option<&ForexPair> {
        self.forex_pairs.get(pair)
    }

    fn get_all(&self) -> Vec<&ForexPair> {
        self.forex_pairs.values().collect()
    }
}

struct AppState {
    db: Mutex<Database>,
    http_client: HttpClient,
}

async fn fetch_forex_price(pair: &str, client: &HttpClient) -> Result<f64, reqwest::Error> {
    let url = format!("https://api.exchangerate.host/convert?from={}&to=USD", pair);
    let response = client.get(&url).send().await?.json::<HashMap<String, f64>>().await?;
    Ok(*response.get("result").unwrap_or(&0.0))
}

async fn get_forex_price(app_state: web::Data<AppState>, pair: web::Path<String>) -> impl Responder {
    let db = app_state.db.lock().unwrap();
    match db.get(&pair) {
        Some(forex_pair) => HttpResponse::Ok().json(forex_pair),
        None => HttpResponse::NotFound().finish(),
    }
}

async fn get_all_forex_prices(app_state: web::Data<AppState>) -> impl Responder {
    let db = app_state.db.lock().unwrap();
    let forex_pairs = db.get_all();
    HttpResponse::Ok().json(forex_pairs)
}

async fn update_forex_prices(app_state: web::Data<AppState>) -> impl Responder {
    let forex_pairs = vec!["EURUSD", "GBPUSD", "USDJPY"];
    let client = &app_state.http_client;
    let mut db = app_state.db.lock().unwrap();

    for pair in forex_pairs {
        if let Ok(price) = fetch_forex_price(pair, client).await {
            db.insert(ForexPair {
                pair: pair.to_string(),
                price,
            });
        }
    }

    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(AppState {
        db: Mutex::new(Database::new()),
        http_client: HttpClient::new(),
    });

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::permissive()
                    .allowed_origin_fn(|origin, _req_head| {
                        origin.as_bytes().starts_with(b"http://localhost") || origin == "null"
                    })
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .app_data(data.clone())
            .route("/forex/{pair}", web::get().to(get_forex_price))
            .route("/forex", web::get().to(get_all_forex_prices))
            .route("/update_forex", web::post().to(update_forex_prices))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}