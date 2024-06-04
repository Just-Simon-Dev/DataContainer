use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DataSelectionRequest {
    pub select_columns: String,
    pub tables: String,
    pub where_conditions: String,
}