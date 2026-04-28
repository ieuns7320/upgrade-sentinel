use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Finding {
    pub id: String,
    pub severity: String,
    pub title: String,
    pub description: String,
    pub contract_name: String,
    pub function_name: Option<String>,
    pub evidence: Vec<String>,
}
