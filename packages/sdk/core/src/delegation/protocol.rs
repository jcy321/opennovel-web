use super::types::{DelegationRequest, DelegationResult};

pub struct DelegationProtocol;

impl DelegationProtocol {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn delegate(&self, _request: DelegationRequest) -> DelegationResult {
        DelegationResult {
            success: true,
            session_id: uuid::Uuid::new_v4().to_string(),
            output: None,
        }
    }
}

impl Default for DelegationProtocol {
    fn default() -> Self {
        Self::new()
    }
}