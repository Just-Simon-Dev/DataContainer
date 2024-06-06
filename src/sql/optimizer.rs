use sqlparser::ast::{Expr, SelectItem, SetExpr, Statement, TableFactor};

enum LogicalPlanTypes {
    Scan,
    Filter,
    Project
}

struct LogicalPlanNodes {
    logical_plan_nodes: LogicalPlanNodes,
    table: String,
    filters: Vec<String>,
    projections: Vec<SelectItem>
}

struct LogicalPlan {
    nodes: Vec<LogicalPlanNodes>
}

pub fn generate_plans(queries: Vec<Statement>){ 
    let mut logical_plan_nodes: LogicalPlan;

    for statement in queries {
        if let Statement::Query(query) = statement {
            if let SetExpr::Select(select) = &*query.body {
                for table_with_joins in &select.from {
                    if let TableFactor::Table { name, .. } = &table_with_joins.relation {
                        let scan = LogicalPlanNodes {
                            logical_plan_nodes: LogicalPlanNodes::Scan,
                            table: name.to_string(),
                            filters: vec![],
                            projections: vec![]
                        };
                        logical_plan_nodes.nodes.push(scan);
                    }
                    for selection in &select.selection {
                        let filter = LogicalPlanNodes {
                            logical_plan_nodes: LogicalPlanNodes::Filter,
                            table: "".to_string(),
                            filters: vec![selection.to_string()],
                            projections: vec![],
                        };
                        logical_plan_nodes.nodes.push(filter)
                    }
                    for projection in select.projection.clone() {
                        let proj = LogicalPlanNodes {
                            logical_plan_nodes: LogicalPlanNodes::Project,
                            table: "".to_string(),
                            filters: vec![],
                            projections: vec![projection],
                        };
                        logical_plan_nodes.nodes.push(proj);
                    }
                }
            }
        }
    }
    
    for scan in logical_plan_nodes {
        for projection in &scan.projections {
            println!("{}", projection);
        }
        println!("Table: {}, Filters: {:?}, Projections: {:?}", scan.table, scan.filters, scan.projections);
    }
}

fn extract_filters(selection: &Option<Expr>, table_name: &str) -> Vec<String> {
    let mut filters = Vec::new();
    if let Some(expr) = selection {
        let filter_str = format!("{}", expr);
        if filter_str.contains(table_name) {
            filters.push(filter_str);
        }
    }
    filters
}

