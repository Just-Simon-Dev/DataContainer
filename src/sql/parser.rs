use sqlparser::ast::Statement;
use sqlparser::dialect::Dialect;
use sqlparser::parser::Parser;

pub fn sql_to_ast_translation(sql_query: &str, dialect: impl Dialect) -> Vec<Statement>{
    
    let ast_expression = Parser::parse_sql(
        &dialect,
        &*sql_query
    ).unwrap();
    
    return  ast_expression;
}