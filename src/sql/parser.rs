use sqlparser::ast::{
    Expr, GroupByExpr, Ident, ObjectName, Select, SelectItem, SetExpr, Statement, TableFactor, TableWithJoins,
};
use sqlparser::dialect::Dialect;
use sqlparser::parser::Parser;
use sqlparser::tokenizer::Tokenizer;
use crate::api::requests::DataSelectionRequest;

pub fn sql_to_ast_translation(sql_query: &str, dialect: &impl Dialect) -> Vec<Statement> {
    Parser::parse_sql(dialect, sql_query).unwrap()
}

pub fn request_to_ast_translation(req: DataSelectionRequest, dialect: impl Dialect) -> Vec<Statement>{
    let select_items: Vec<SelectItem> = req.select_columns
        .split(',')
        .map(|col| SelectItem::UnnamedExpr(Expr::Identifier(Ident::new(col.trim()))))
        .collect();
    
    if select_items.is_empty() || select_items[0].to_string() == ""{
        panic!("The select columns cannot be empty");
    }
    
    let table_factors: Vec<TableWithJoins> = req.tables
        .split(',')
        .map(|table| TableWithJoins {
            relation: TableFactor::Table {
                name: ObjectName(vec![Ident::new(table.trim())]),
                alias: None,
                args: None,
                with_hints: vec![],
                version: None,
                partitions: vec![],
            },
            joins: vec![],
        })
        .collect();

    if table_factors.is_empty() || table_factors[0].to_string() == ""{
        panic!("The tables cannot be empty");
    }
    
    let mut tokenizer = Tokenizer::new(&dialect, &*req.where_conditions);
    let tokens = tokenizer.tokenize().unwrap();
    let parser = Parser::new(&dialect);
    let where_expr = if tokens.len() > 0 {
        Some(parser.with_tokens(tokens).parse_expr().unwrap())
    } else {
        None
    };
    

    let select = Select {
        distinct: None,
        top: None,
        projection: select_items,
        into: None,
        from: table_factors,
        lateral_views: vec![],
        selection: where_expr,
        group_by: GroupByExpr::Expressions(vec![]),
        cluster_by: vec![],
        distribute_by: vec![],
        sort_by: vec![],
        having: None,
        named_window: vec![],
        qualify: None,
        window_before_qualify: false,
        value_table_mode: None,
        connect_by: None,
    };

    let query = SetExpr::Select(Box::new(select));
    vec![Statement::Query(Box::new(sqlparser::ast::Query {
        with: None,
        body: Box::from(query),
        order_by: vec![],
        limit: None,
        limit_by: vec![],
        offset: None,
        fetch: None,
        locks: vec![],
        for_clause: None,
    }))]
}