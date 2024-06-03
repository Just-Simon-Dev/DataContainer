mod config;
mod api;
pub(crate) mod sql;

use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use log::info;
use config::settings;
use std::env;
use crate::api::routes::configure_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    // Load configuration
    let settings = settings::load_settings();
    let config: settings::ServerSettings = settings.server;

    // Log the starting information
    info!("Starting server on port {}", config.port);

    // Start HTTP server
    HttpServer::new(|| {
        App::new().configure(configure_routes)
    })
        .bind(("127.0.0.1", config.port))?
        .run()
        .await
}
