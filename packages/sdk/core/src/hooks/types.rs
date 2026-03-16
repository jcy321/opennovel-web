use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HookPoint {
    BeforeUserMessage,
    AfterUserMessage,
    BeforeAssistantMessage,
    AfterAssistantMessage,
    BeforeToolCall,
    AfterToolResult,
    OnSessionStart,
    OnSessionEnd,
    OnAgentSwitch,
    OnPhaseChange,
    OnChapterComplete,
    OnConsistencyViolation,
    OnError,
    OnIdle,
}

#[derive(Debug, Clone)]
pub struct HookContext {
    pub session_id: String,
    pub book_id: String,
    pub agent_name: String,
    pub message: Option<String>,
    pub data: HashMap<String, serde_json::Value>,
}

impl HookContext {
    pub fn new(session_id: String, book_id: String, agent_name: String) -> Self {
        Self {
            session_id,
            book_id,
            agent_name,
            message: None,
            data: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HookResult {
    pub continue_execution: bool,
    pub modified_message: Option<String>,
}

impl HookResult {
    pub fn cont() -> Self {
        Self {
            continue_execution: true,
            modified_message: None,
        }
    }

    pub fn stop() -> Self {
        Self {
            continue_execution: false,
            modified_message: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HookDefinition {
    pub name: String,
    pub point: HookPoint,
    pub priority: u32,
    pub enabled: bool,
}

impl HookDefinition {
    pub fn new(name: impl Into<String>, point: HookPoint) -> Self {
        Self {
            name: name.into(),
            point,
            priority: 100,
            enabled: true,
        }
    }
}
