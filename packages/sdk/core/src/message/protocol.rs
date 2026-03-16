use super::StreamEvent;

pub struct SSEProtocol;

impl SSEProtocol {
    pub fn serialize(event: &StreamEvent) -> Result<String, serde_json::Error> {
        let json = serde_json::to_string(event)?;
        Ok(format!("data: {}\n\n", json))
    }

    pub fn parse(data: &str) -> Result<StreamEvent, serde_json::Error> {
        let json = data.strip_prefix("data: ").unwrap_or(data);
        serde_json::from_str(json)
    }
}

impl StreamEvent {
    pub fn thinking_start(agent: impl Into<String>) -> Self {
        Self::ThinkingStart {
            agent: agent.into(),
        }
    }

    pub fn thinking_delta(agent: impl Into<String>, delta: impl Into<String>) -> Self {
        Self::ThinkingDelta {
            agent: agent.into(),
            delta: delta.into(),
        }
    }

    pub fn thinking_end(agent: impl Into<String>) -> Self {
        Self::ThinkingEnd {
            agent: agent.into(),
        }
    }

    pub fn content_start(agent: impl Into<String>) -> Self {
        Self::ContentStart {
            agent: agent.into(),
        }
    }

    pub fn content_delta(agent: impl Into<String>, delta: impl Into<String>) -> Self {
        Self::ContentDelta {
            agent: agent.into(),
            delta: delta.into(),
        }
    }

    pub fn content_end(agent: impl Into<String>) -> Self {
        Self::ContentEnd {
            agent: agent.into(),
        }
    }

    pub fn complete() -> Self {
        Self::Complete
    }

    pub fn error(agent: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Error {
            agent: agent.into(),
            message: message.into(),
        }
    }
}
