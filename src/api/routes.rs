use actix_web::web;

// Import handler functions
use crate::api::handlers::*;

// Define API routes and associate them with handler functions
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::get().to(index))
    ).service(
        web::resource("/echo")
            .route(web::post().to(echo))
    ).service(
        web::resource("/get")
            .route(web::get().to(data_selection))
    );
}