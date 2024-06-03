#[path = "../src/sql/mod.rs"]
mod sql;

#[cfg(test)]
mod unit_tests{
    use sqlparser::dialect::MySqlDialect;
    use crate::sql::parser;

    #[test]
    fn test_sql_to_ast_parsing(){
        let dialect = MySqlDialect {};

        let sql_expression: &str = "SELECT * FROM users";
        
        parser::sql_to_ast_translation(sql_expression, dialect);
        
        assert!(true)
    }
}