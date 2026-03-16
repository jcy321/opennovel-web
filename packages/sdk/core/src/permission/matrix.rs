use super::{Permission, PermissionSet};

#[derive(Debug, Clone)]
pub struct PermissionMatrix {
    matrix: std::collections::HashMap<String, PermissionSet>,
}

impl PermissionMatrix {
    pub fn new() -> Self {
        Self {
            matrix: std::collections::HashMap::new(),
        }
    }

    pub fn check(&self, role: &str, permission: Permission) -> bool {
        self.matrix
            .get(role)
            .map(|ps| ps.contains(permission))
            .unwrap_or(false)
    }

    pub fn grant(&mut self, role: &str, permission: Permission) {
        self.matrix
            .entry(role.to_string())
            .or_default()
            .add(permission);
    }
}

impl Default for PermissionMatrix {
    fn default() -> Self {
        Self::new()
    }
}
