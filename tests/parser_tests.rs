#[path = "../src/sql/mod.rs"]
mod sql;
#[path = "../src/api/mod.rs"]
mod api;

#[cfg(test)]
mod parser_tests {
    use sqlparser::dialect::MySqlDialect;
    use crate::api::handlers::DataSelectionRequest;
    use crate::sql::parser;

    #[test]
    fn test_sql_to_ast_parsing(){
        let dialect = MySqlDialect {};

        let sql_expression: &str = "SELECT * FROM users";

        parser::sql_to_ast_translation(sql_expression, &dialect);

        assert!(true)
    }

    #[test]
    #[should_panic]
    fn test_request_to_ast_parsing_without_data(){
        let dialect = MySqlDialect {};

        let request_expression: DataSelectionRequest = DataSelectionRequest {
            select_columns: "".to_string(),
            tables: "".to_string(),
            where_conditions: "".to_string(),
        };

        let data = parser::request_to_ast_translation(request_expression, dialect);

        for statement in data {
            println!("{}", statement);
        }

        assert!(true)
    }
    #[test]
    #[should_panic]
    fn test_request_to_ast_parsing_witout_columns(){
        let dialect = MySqlDialect {};

        let request_expression: DataSelectionRequest = DataSelectionRequest {
            select_columns: "".to_string(),
            tables: "users".to_string(),
            where_conditions: "id=1".to_string(),
        };

        parser::request_to_ast_translation(request_expression, dialect);

        assert!(true)
    }
    #[test]
    #[should_panic]
    fn test_request_to_ast_parsing_without_tables(){
        let dialect = MySqlDialect {};

        let request_expression: DataSelectionRequest = DataSelectionRequest {
            select_columns: "*".to_string(),
            tables: "".to_string(),
            where_conditions: "id=1".to_string(),
        };

        parser::request_to_ast_translation(request_expression, dialect);

        assert!(true)
    }
    #[test]
    fn test_request_to_ast_parsing_without_where(){
        let dialect = MySqlDialect {};

        let request_expression: DataSelectionRequest = DataSelectionRequest {
            select_columns: "*".to_string(),
            tables: "users".to_string(),
            where_conditions: "".to_string(),
        };

        parser::request_to_ast_translation(request_expression, dialect);

        assert!(true)
    }
    #[test]
    fn test_request_to_ast_parsing_with_all_data(){
        let dialect = MySqlDialect {};

        let request_expression: DataSelectionRequest = DataSelectionRequest {
            select_columns: "*".to_string(),
            tables: "users".to_string(),
            where_conditions: "id=1".to_string(),
        };

        parser::request_to_ast_translation(request_expression, dialect);

        assert!(true)
    }
}