use serde::{Deserialize, Serialize};
use tscanner_types::Severity;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RuleExecutionKind {
    Ast,
    Regex,
}

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
    pub rule_type: RuleExecutionKind,
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
            rule_type: RuleExecutionKind::Ast,
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

impl RuleCategory {
    pub fn as_folder_name(&self) -> &'static str {
        match self {
            RuleCategory::TypeSafety => "type_safety",
            RuleCategory::CodeQuality => "code_quality",
            RuleCategory::Style => "style",
            RuleCategory::Performance => "performance",
            RuleCategory::BugPrevention => "bug_prevention",
            RuleCategory::Variables => "variables",
            RuleCategory::Imports => "imports",
        }
    }
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
