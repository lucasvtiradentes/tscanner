use core::types::Severity;

pub struct RuleInfo {
    pub name: String,
    pub enabled: bool,
    pub severity: Severity,
    pub pattern: Option<String>,
}

impl RuleInfo {
    pub fn is_custom(&self) -> bool {
        self.pattern.is_some()
    }
}
