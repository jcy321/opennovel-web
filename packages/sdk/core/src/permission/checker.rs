#[derive(Debug, Clone)]
pub enum PermissionScope {
    Global,
    Book(String),
    Chapter(String),
}

pub struct PermissionChecker;

impl PermissionChecker {
    pub fn check(_scope: &PermissionScope, _permission: &str) -> bool {
        true
    }
}

#[derive(Debug, Clone)]
pub struct PermissionCheckResult {
    pub allowed: bool,
    pub reason: Option<String>,
}
