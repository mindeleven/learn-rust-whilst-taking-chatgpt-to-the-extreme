/// code along with https://actix.rs/docs/application/
/// 
/// examples of primitives actix-web provides to build web servers and applications with Rust:
/// -> routing, middleware, pre-processing of requests, post-processing of responses, etc.
/// 
/// an actix-web server is built around an App instance which is used for 
/// -> registering routes for resources and middleware
/// -> storing application state shared across all handlers within the same scope
/// 
/// application's scope:
/// -> acts as a namespace for all routes
/// -> all routes for a specific application scope have the same url path prefix
/// -> application prefix always contains a leading "/" 
/// -> the prefix should consist of value path segments
/// 
/// example: an application with the /app prefix and an index.html
/// 
use actix_web::{
    get, web, App, HttpServer, Responder
};

/// State
/// -> shared with all routes and resources within the same scope
/// -> can be accessed with the web::Data<T> extractor where T is the type of the state
/// -> also accessible for middleware
/// example: storing the application name in the state
struct AppState {
    app_name: String
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello world from {}!", app_name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    HttpServer::new(|| {
        /* App::new().service(
            // scope() prefixes all resources and routes attached to it
            web::scope("/app")
                // route() to handle requests for `GET /app/index.html`
                .route("/index.html", web::get().to(index)),
        ) */
        // passing in the state when initializing the App
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix web"),
            }))
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run() // runs at http://127.0.0.1:8080/app/index.html
    .await

}
