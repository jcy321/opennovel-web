use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegationRequest {
    pub task: String,
    pub expected_outcome: ExpectedOutcome,
    pub required_tools: Vec<String>,
    pub must_do: Vec<String>,
    pub must_not_do: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpectedOutcome {
    pub deliverables: Vec<String>,
    pub success_criteria: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegationResult {
    pub success: bool,
    pub session_id: String,
    pub output: Option<serde_json::Value>,
}
