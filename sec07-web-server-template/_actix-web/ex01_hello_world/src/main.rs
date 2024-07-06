/// docs @ https://actix.rs/docs/getting-started/
/// 
use actix_web::{
    get, 
    post, 
    web, 
    App, 
    HttpResponse, 
    HttpServer, 
    Responder
};

// defining the request handlers
// request handlers are async functions that can be parameterized
// routing information is attached directly using the built-in macros
#[get("/")] // attribute like procedural macro
async fn hello() -> impl Responder {
    // returns http response
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")] // endpoint for sending data
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

// route that does not use a routing macro
// needs to be registered in main
async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

// creating an App instance
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // registering the request handlers
    HttpServer::new(|| {
        App::new()
            // App::service is for the handlers using routing macros
            .service(hello)
            .service(echo)
            // App::route is for manually routed handlers
            // it declares the path and the method
            .route("/hey", web::get().to(manual_hello))
    })
    // app is starting inside an HttpServer 
    // this server will serve incoming requests using the App as an application factory
    .bind(("127.0.0.1", 8080))?
    .run() // runs at http://127.0.0.1:8080/
    .await
}
