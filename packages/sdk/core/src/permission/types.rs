use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Permission {
    #[serde(rename = "read_knowledge")]
    ReadKnowledge,
    #[serde(rename = "write_knowledge")]
    WriteKnowledge,
    #[serde(rename = "create_annotation")]
    CreateAnnotation,
    #[serde(rename = "modify_annotation")]
    ModifyAnnotation,
    #[serde(rename = "lock_agent")]
    LockAgent,
    #[serde(rename = "unlock_agent")]
    UnlockAgent,
    #[serde(rename = "intervene")]
    Intervene,
    #[serde(rename = "modify_outline")]
    ModifyOutline,
    #[serde(rename = "write_content")]
    WriteContent,
    #[serde(rename = "run_analysis")]
    RunAnalysis,
    #[serde(rename = "sync_data")]
    SyncData,
}

#[derive(Debug, Clone, Default)]
pub struct PermissionSet {
    permissions: HashSet<Permission>,
}

impl PermissionSet {
    pub fn new() -> Self {
        Self {
            permissions: HashSet::new(),
        }
    }

    pub fn add(&mut self, permission: Permission) {
        self.permissions.insert(permission);
    }

    pub fn contains(&self, permission: Permission) -> bool {
        self.permissions.contains(&permission)
    }
}
