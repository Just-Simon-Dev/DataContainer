#[path = "../src/sql/mod.rs"]
mod sql;
#[path = "../src/api/mod.rs"]
mod api;

#[cfg(test)]
mod optimizer_tests {
    use sqlparser::dialect::MySqlDialect;
    use crate::sql::parser;
    use crate::sql::optimizer;
    use crate::sql::optimizer::LogicalPlanNode;

    #[test]
    fn test_multiple_tables(){
        let dialect = MySqlDialect {};

        let sql_expression: &str = "SELECT users.*, customer.* FROM users, customers";

        let ast = parser::sql_to_ast_translation(sql_expression, &dialect);
        let plans = optimizer::generate_plans(ast);

        assert_eq!(plans.nodes.iter().count(), 2)
    }

    #[test]
    fn test_multiple_projections() {
        let dialect = MySqlDialect {};

        let sql_expression: &str = "SELECT users.mask, users.id FROM users";

        let ast = parser::sql_to_ast_translation(sql_expression, &dialect);
        let plans = optimizer::generate_plans(ast);
        
        let node: &LogicalPlanNode = plans.nodes.iter().collect::<Vec<_>>()[0];

        assert_eq!(node.projections.iter().count(), 2)
    }
    
    #[test]
    fn test_multiple_conditions() {
        let dialect = MySqlDialect {};

        let sql_expression: &str = "SELECT users.id FROM users WHERE users.id = 1 AND users.mask = 1";

        let ast = parser::sql_to_ast_translation(sql_expression, &dialect);
        let plans = optimizer::generate_plans(ast);

        let node: &LogicalPlanNode = plans.nodes.iter().collect::<Vec<_>>()[0];

        assert_eq!(node.filters.iter().count(), 2)
    }
}