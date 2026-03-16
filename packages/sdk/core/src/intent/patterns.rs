use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntentPattern {
    HowTo,
    Create,
    Find,
    Fix,
    Evaluate,
}

pub struct IntentPatternLibrary;

impl IntentPatternLibrary {
    pub fn match_pattern(_message: &str) -> Option<IntentPattern> {
        None
    }
}

pub use super::gate::IntentType;
