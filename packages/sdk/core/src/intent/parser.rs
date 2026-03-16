use super::IntentType;

pub struct ParsedIntent {
    pub intent_type: IntentType,
    pub confidence: f32,
}

pub struct IntentParser;

impl IntentParser {
    pub fn parse(_message: &str) -> ParsedIntent {
        ParsedIntent {
            intent_type: IntentType::OpenEnded,
            confidence: 0.5,
        }
    }
}
