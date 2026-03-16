use super::{Agent, AgentId, AgentRole, CreationStage};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Default)]
pub struct AgentRegistry {
    agents: HashMap<AgentId, Arc<dyn Agent>>,
    role_index: HashMap<AgentRole, Vec<AgentId>>,
}

impl AgentRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, agent: Arc<dyn Agent>) {
        let id = agent.id().clone();
        let role = agent.role();

        self.agents.insert(id.clone(), agent);
        self.role_index.entry(role).or_default().push(id);
    }

    pub fn get(&self, id: &AgentId) -> Option<Arc<dyn Agent>> {
        self.agents.get(id).cloned()
    }

    pub fn get_by_role(&self, role: AgentRole) -> Vec<Arc<dyn Agent>> {
        self.role_index
            .get(&role)
            .map(|ids| ids.iter().filter_map(|id| self.get(id)).collect())
            .unwrap_or_default()
    }

    pub fn all(&self) -> Vec<Arc<dyn Agent>> {
        self.agents.values().cloned().collect()
    }

    pub fn len(&self) -> usize {
        self.agents.len()
    }

    pub fn is_empty(&self) -> bool {
        self.agents.is_empty()
    }

    pub fn get_available_agents(&self, stage: CreationStage) -> Vec<Arc<dyn Agent>> {
        self.agents
            .values()
            .filter(|agent| agent.is_available_at(stage))
            .cloned()
            .collect()
    }

    pub fn is_agent_available(&self, id: &AgentId, stage: CreationStage) -> bool {
        self.agents
            .get(id)
            .map(|agent| agent.is_available_at(stage))
            .unwrap_or(false)
    }
}
