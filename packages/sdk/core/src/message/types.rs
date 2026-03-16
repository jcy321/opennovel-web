use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MessageId(String);

impl MessageId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for MessageId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AnnotationId(String);

impl AnnotationId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextRange {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MessageSender {
    #[serde(rename = "user")]
    User,

    #[serde(rename = "agent")]
    Agent { id: String },

    #[serde(rename = "system")]
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: MessageId,
    pub sender: MessageSender,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    #[serde(default)]
    pub annotations: Vec<AnnotationId>,
}

impl ChatMessage {
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            id: MessageId::new(),
            sender: MessageSender::User,
            content: content.into(),
            timestamp: Utc::now(),
            annotations: vec![],
        }
    }

    pub fn agent(agent_id: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            id: MessageId::new(),
            sender: MessageSender::Agent {
                id: agent_id.into(),
            },
            content: content.into(),
            timestamp: Utc::now(),
            annotations: vec![],
        }
    }

    pub fn system(content: impl Into<String>) -> Self {
        Self {
            id: MessageId::new(),
            sender: MessageSender::System,
            content: content.into(),
            timestamp: Utc::now(),
            annotations: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum StreamEvent {
    #[serde(rename = "thinking_start")]
    ThinkingStart { agent: String },

    #[serde(rename = "thinking_delta")]
    ThinkingDelta { agent: String, delta: String },

    #[serde(rename = "thinking_end")]
    ThinkingEnd { agent: String },

    #[serde(rename = "content_start")]
    ContentStart { agent: String },

    #[serde(rename = "content_delta")]
    ContentDelta { agent: String, delta: String },

    #[serde(rename = "content_end")]
    ContentEnd { agent: String },

    #[serde(rename = "annotation")]
    Annotation {
        id: String,
        agent: String,
        position: TextRange,
        content: String,
        severity: String,
    },

    #[serde(rename = "intervention")]
    Intervention { agent: String, message: String },

    #[serde(rename = "stage_change")]
    StageChange { from: String, to: String },

    #[serde(rename = "complete")]
    Complete,

    #[serde(rename = "error")]
    Error { agent: String, message: String },
}
