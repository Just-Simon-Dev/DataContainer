mod config;

use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use log::info;
use config::settings;
use std::env;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    // Load configuration
    let settings = settings::load_settings().unwrap();
    let config: settings::ServerSettings = settings.server;

    // Log the starting information
    info!("Starting server on port {}", config.port);

    // Start HTTP server
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/echo", web::post().to(echo))
    })
        .bind(("127.0.0.1", config.port))?
        .run()
        .await
}
