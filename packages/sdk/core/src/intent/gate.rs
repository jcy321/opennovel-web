use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntentType {
    Research,
    Implementation,
    Exploration,
    Evaluation,
    Fix,
    OpenEnded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedIntent {
    pub intent_type: IntentType,
    pub confidence: f32,
    pub entities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RouteTarget {
    Agent(String),
    Tool(String),
    Clarification(String),
}

pub struct IntentGate;

impl IntentGate {
    pub fn parse(message: &str) -> ParsedIntent {
        let intent_type = Self::classify(message);
        ParsedIntent {
            intent_type,
            confidence: 0.9,
            entities: vec![],
        }
    }

    fn classify(message: &str) -> IntentType {
        let msg = message.to_lowercase();
        if msg.contains("如何") || msg.contains("怎么") || msg.contains("什么是") {
            IntentType::Research
        } else if msg.contains("写") || msg.contains("创建") || msg.contains("添加") {
            IntentType::Implementation
        } else if msg.contains("查找") || msg.contains("搜索") || msg.contains("分析") {
            IntentType::Exploration
        } else if msg.contains("修复") || msg.contains("修改") || msg.contains("改") {
            IntentType::Fix
        } else {
            IntentType::OpenEnded
        }
    }
}
