use sqlparser::ast::{SetExpr, Statement, TableFactor};

#[derive(PartialEq, Clone)]
enum LogicalPlanTypes {
    Scan,
    Filter,
    Project
}

#[derive(Clone)]
pub struct LogicalPlanNode {
    pub logical_plan_type: LogicalPlanTypes,
    pub table: String,
    pub filters: Vec<String>,
    pub projections: Vec<String>
}

pub struct LogicalPlan {
    pub nodes: Vec<LogicalPlanNode>
}

impl LogicalPlan {
    pub fn optimize(&mut self) {
        // Collect optimized nodes
        let mut optimized_nodes: Vec<LogicalPlanNode> = vec![];
        let mut scan_nodes: Vec<LogicalPlanNode> = vec![];

        // First, collect all scan nodes
        for node in &self.nodes {
            if LogicalPlanTypes::Scan == node.logical_plan_type {
                scan_nodes.push(node.clone());
            }
        }

        // Now, optimize each scan node
        for scan_node in scan_nodes {
            let new_node = self.optimize_scan_node(&scan_node);
            optimized_nodes.push(new_node);
        }

        // Update the nodes with optimized nodes
        self.nodes = optimized_nodes;
    }
    fn optimize_scan_node(&mut self, scan_node: &LogicalPlanNode) -> LogicalPlanNode {
        let mut optimized_node: LogicalPlanNode = LogicalPlanNode {
            logical_plan_type: LogicalPlanTypes::Scan,
            table: scan_node.table.clone(),
            filters: scan_node.filters.clone(),
            projections: scan_node.projections.clone(),
        };
        
        for node in &self.nodes {
            if node.table == scan_node.table {
                match node.logical_plan_type { 
                    LogicalPlanTypes::Filter => {
                        for filter in &node.filters {
                            optimized_node.filters.push(filter.to_string());
                        }
                    }
                    LogicalPlanTypes::Project => {
                        for projection in &node.projections {
                            optimized_node.projections.push(projection.clone());
                        }
                    }
                    _ => {}
                }
                
            }
        }
        
        optimized_node
    }
}

pub fn generate_plans(queries: Vec<Statement>) -> LogicalPlan{
    let mut logical_plan_nodes: LogicalPlan = LogicalPlan {
        nodes: vec![],
    };

    for statement in queries {
        if let Statement::Query(query) = statement {
            if let SetExpr::Select(select) = &*query.body {
                for table_with_joins in &select.from {
                    if let TableFactor::Table { name, .. } = &table_with_joins.relation {
                        let scan = LogicalPlanNode {
                            logical_plan_type: LogicalPlanTypes::Scan,
                            table: name.to_string(),
                            filters: vec![],
                            projections: vec![]
                        };
                        logical_plan_nodes.nodes.push(scan);
                    }
                }
                for selection in &select.selection {
                    let filter = LogicalPlanNode {
                        logical_plan_type: LogicalPlanTypes::Filter,
                        table: selection.to_string().split(".").collect::<Vec<_>>()[0].to_string(),
                        filters: vec![selection.to_string()],
                        projections: vec![],
                    };
                    logical_plan_nodes.nodes.push(filter)
                }
                for projection in select.projection.clone() {
                    let proj = LogicalPlanNode {
                        logical_plan_type: LogicalPlanTypes::Project,
                        table: projection.to_string().split(".").collect::<Vec<_>>()[0].to_string(),
                        filters: vec![],
                        projections: vec![projection.to_string()],
                    };
                    logical_plan_nodes.nodes.push(proj);
                }
            }
        }
    }
    
    logical_plan_nodes.optimize();
    
    logical_plan_nodes
}


