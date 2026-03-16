use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RouteTarget {
    Agent(String),
    Tool(String),
    Clarification(String),
}

pub struct IntentRouter;

impl IntentRouter {
    pub fn route(_intent: &str) -> RouteTarget {
        RouteTarget::Agent("tian-dao".to_string())
    }
}
