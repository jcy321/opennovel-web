use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::session::SessionContext;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AgentId(String);

impl AgentId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for AgentId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentRole {
    Planner,
    TianDao,
    WorldGuardian,
    LiuHeping,
    Writer,
    Reviewer,
    Observer,
    Researcher,
}

impl AgentRole {
    pub fn can_intervene(&self) -> bool {
        matches!(self, AgentRole::TianDao | AgentRole::WorldGuardian)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCapability {
    pub name: String,
    pub description: String,
    pub triggers: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct AgentResponse {
    pub content: String,
    pub thinking: Option<String>,
    pub annotations: Vec<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum AgentError {
    #[error("Provider错误: {0}")]
    ProviderError(String),
    
    #[error("权限不足")]
    PermissionDenied,
    
    #[error("处理失败: {0}")]
    ProcessingFailed(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CreationStage {
    Outline,
    WorldBuilding,
    CharacterDevelopment,
    Writing,
    Revision,
    Publishing,
}

#[async_trait]
pub trait Agent: Send + Sync {
    fn id(&self) -> &AgentId;
    
    fn name(&self) -> &str;
    
    fn role(&self) -> AgentRole;
    
    fn capabilities(&self) -> &[AgentCapability];
    
    async fn process(
        &self,
        message: &str,
        context: &SessionContext,
    ) -> Result<AgentResponse, AgentError>;
    
    fn can_intervene(&self) -> bool {
        self.role().can_intervene()
    }
    
    fn system_prompt(&self) -> &str;
    
    fn tools(&self) -> Vec<String> {
        vec![]
    }
    
    fn hooks(&self) -> Vec<String> {
        vec![]
    }
    
    fn available_stages(&self) -> Option<Vec<CreationStage>> {
        None
    }
    
    fn is_available_at(&self, stage: CreationStage) -> bool {
        match self.available_stages() {
            None => true,
            Some(stages) => stages.contains(&stage),
        }
    }
}