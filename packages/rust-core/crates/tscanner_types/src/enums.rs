use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Error,
    #[default]
    Warning,
    Info,
    Hint,
}

impl Severity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Info => "info",
            Severity::Hint => "hint",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum IssueRuleType {
    #[default]
    Builtin,
    CustomRegex,
    CustomScript,
    Ai,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RuleType {
    Ast,
    Regex,
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AiProvider {
    Claude,
    Codex,
    Gemini,
}

impl AiProvider {
    pub const ALL: &'static [AiProvider] =
        &[AiProvider::Claude, AiProvider::Codex, AiProvider::Gemini];

    pub fn as_str(&self) -> &'static str {
        match self {
            AiProvider::Claude => "claude",
            AiProvider::Codex => "codex",
            AiProvider::Gemini => "gemini",
        }
    }

    pub fn all_names() -> String {
        Self::ALL
            .iter()
            .map(|p| p.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    }
}

impl FromStr for AiProvider {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "claude" => Ok(AiProvider::Claude),
            "codex" => Ok(AiProvider::Codex),
            "gemini" => Ok(AiProvider::Gemini),
            _ => Err(format!(
                "unsupported AI provider '{}'. Available providers: {}",
                value,
                AiProvider::all_names()
            )),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum AiMode {
    #[default]
    Paths,
    Content,
    Agentic,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum AiExecutionMode {
    #[default]
    Ignore,
    Include,
    Only,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum StartupScanMode {
    Off,
    #[default]
    Cached,
    Fresh,
}

impl StartupScanMode {
    pub fn from_str_or_panic(s: &str) -> Self {
        match s {
            "off" => Self::Off,
            "cached" => Self::Cached,
            "fresh" => Self::Fresh,
            _ => panic!("Invalid StartupScanMode in constants.json: '{}'", s),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanMode {
    Codebase,
    Branch,
    Staged,
    Uncommitted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroupMode {
    File,
    Rule,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    List,
    Tree,
}
