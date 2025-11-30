use crate::output::Severity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RuleType {
    Ast,
    Regex,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleMetadata {
    pub name: &'static str,
    pub display_name: &'static str,
    pub description: &'static str,
    pub rule_type: RuleType,
    pub default_severity: Severity,
    pub default_enabled: bool,
    pub category: RuleCategory,
    #[serde(default)]
    pub typescript_only: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub equivalent_eslint_rule: Option<&'static str>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub equivalent_biome_rule: Option<&'static str>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RuleCategory {
    TypeSafety,
    CodeQuality,
    Style,
    Performance,
    BugPrevention,
    Variables,
    Imports,
}

pub struct RuleMetadataRegistration {
    pub metadata: RuleMetadata,
}

inventory::collect!(RuleMetadataRegistration);

pub fn get_all_rule_metadata() -> Vec<RuleMetadata> {
    inventory::iter::<RuleMetadataRegistration>()
        .map(|reg| reg.metadata.clone())
        .collect()
}
