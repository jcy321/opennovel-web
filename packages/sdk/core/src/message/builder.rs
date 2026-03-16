use super::{ChatMessage, MessageSender};
use chrono::{DateTime, Utc};

pub struct MessageBuilder {
    sender: Option<MessageSender>,
    content: Option<String>,
    timestamp: Option<DateTime<Utc>>,
}

impl MessageBuilder {
    pub fn new() -> Self {
        Self {
            sender: None,
            content: None,
            timestamp: None,
        }
    }

    pub fn user(mut self) -> Self {
        self.sender = Some(MessageSender::User);
        self
    }

    pub fn agent(mut self, id: impl Into<String>) -> Self {
        self.sender = Some(MessageSender::Agent { id: id.into() });
        self
    }

    pub fn system(mut self) -> Self {
        self.sender = Some(MessageSender::System);
        self
    }

    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    pub fn timestamp(mut self, timestamp: DateTime<Utc>) -> Self {
        self.timestamp = Some(timestamp);
        self
    }

    pub fn build(self) -> Result<ChatMessage, &'static str> {
        let sender = self.sender.ok_or("Sender is required")?;
        let content = self.content.ok_or("Content is required")?;

        Ok(ChatMessage {
            id: super::MessageId::new(),
            sender,
            content,
            timestamp: self.timestamp.unwrap_or_else(Utc::now),
            annotations: vec![],
        })
    }
}

impl Default for MessageBuilder {
    fn default() -> Self {
        Self::new()
    }
}
