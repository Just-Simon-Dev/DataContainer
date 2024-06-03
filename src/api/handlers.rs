use actix_web::{web, HttpResponse, Result, Responder};
use serde::{Deserialize, Serialize};

// Define request and response structs
// #[derive(Debug, Deserialize)]
// pub struct MyRequest {
//     // Define request parameters
// }
// 
// #[derive(Debug, Serialize)]
// pub struct MyResponse {
//     // Define response data
// }
// 
// // Define request handler functions
// pub async fn handle_my_request(req: web::Json<MyRequest>) -> Result<HttpResponse> {
//     // Process request and generate response
//     let response = MyResponse {
//         // Populate response fields
//     };
// 
//     // Return HTTP response with JSON body
//     Ok(HttpResponse::Ok().json(response))
// }

pub  async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

pub  async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}