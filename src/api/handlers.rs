use actix_web::{web, HttpResponse, Responder};
use sqlparser::dialect::MySqlDialect;
use crate::sql::parser::request_to_ast_translation;
use std::string::ToString;
use crate::api::requests::DataSelectionRequest;
use crate::sql::optimizer::generate_plans;

// Define request handler functions
pub async fn data_selection(req: web::Json<DataSelectionRequest>) -> impl Responder {
    let dialect = MySqlDialect {};

    let data = DataSelectionRequest {
        select_columns: req.select_columns.to_string(),
        tables: req.tables.to_string(),
        where_conditions: req.where_conditions.to_string(),
    };

    // Process request and generate response
    let ast_expression = request_to_ast_translation(data, dialect);
    generate_plans(ast_expression);
    
    
    let data: [String; 0] = [];

    // Return HTTP response with JSON body
    HttpResponse::Ok().json(data)
}

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
