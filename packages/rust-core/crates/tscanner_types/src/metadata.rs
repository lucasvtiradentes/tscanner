use crate::enums::{RuleCategory, RuleType, Severity};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum RuleOptionSchema {
    Integer {
        default: i64,
        #[serde(skip_serializing_if = "Option::is_none")]
        minimum: Option<i64>,
    },
    Boolean {
        default: bool,
    },
    String {
        default: &'static str,
    },
    Array {
        items: &'static str,
        default: &'static [&'static str],
    },
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleOption {
    pub name: &'static str,
    pub description: &'static str,
    #[serde(flatten)]
    pub schema: RuleOptionSchema,
}

fn is_empty_options(options: &&'static [RuleOption]) -> bool {
    options.is_empty()
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
    #[serde(default, skip_serializing_if = "is_empty_options", skip_deserializing)]
    pub options: &'static [RuleOption],
}

impl RuleMetadata {
    pub const fn defaults() -> Self {
        Self {
            name: "",
            display_name: "",
            description: "",
            rule_type: RuleType::Ast,
            default_severity: Severity::Warning,
            default_enabled: false,
            category: RuleCategory::CodeQuality,
            typescript_only: false,
            equivalent_eslint_rule: None,
            equivalent_biome_rule: None,
            options: &[],
        }
    }
}
