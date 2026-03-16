pub struct ProviderError(pub String);

#[derive(Debug, Clone)]
pub enum MessageRole {
    System,
    User,
    Assistant,
}

#[derive(Debug, Clone)]
pub struct ChatRequest {
    pub messages: Vec<ChatMessage>,
    pub model: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub role: MessageRole,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct ChatResponse {
    pub content: String,
    pub model: String,
}

#[async_trait::async_trait]
pub trait LLMProvider: Send + Sync {
    fn name(&self) -> &str;
    
    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse, ProviderError>;
}