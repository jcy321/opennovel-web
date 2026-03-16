use super::types::{HookContext, HookDefinition, HookPoint, HookResult};
use std::collections::HashMap;

pub struct HookRegistry {
    hooks: HashMap<HookPoint, Vec<HookDefinition>>,
}

impl HookRegistry {
    pub fn new() -> Self {
        Self {
            hooks: HashMap::new(),
        }
    }

    pub fn register(&mut self, definition: HookDefinition) {
        let point = definition.point.clone();
        self.hooks.entry(point).or_default().push(definition);
    }

    pub fn execute(&self, _point: HookPoint, _context: HookContext) -> HookResult {
        HookResult::cont()
    }
}

impl Default for HookRegistry {
    fn default() -> Self {
        Self::new()
    }
}
