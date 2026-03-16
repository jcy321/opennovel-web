use super::types::{ToolDefinition, ToolResult};
use std::collections::HashMap;

pub struct ToolRegistry {
    tools: HashMap<String, ToolDefinition>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    pub fn register(&mut self, tool: ToolDefinition) {
        self.tools.insert(tool.name.clone(), tool);
    }

    pub fn get(&self, name: &str) -> Option<&ToolDefinition> {
        self.tools.get(name)
    }

    pub fn execute(&self, _name: &str, _args: serde_json::Value) -> ToolResult {
        ToolResult::success("Tool executed")
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}
