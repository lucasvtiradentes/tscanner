use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct ValidationResult {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl ValidationResult {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }

    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
    }

    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }

    pub fn merge(&mut self, other: ValidationResult) {
        self.errors.extend(other.errors);
        self.warnings.extend(other.warnings);
    }
}

const ALLOWED_TOP_LEVEL: &[&str] = &[
    "$schema",
    "codeEditor",
    "cli",
    "builtinRules",
    "customRules",
    "files",
];
const ALLOWED_CLI: &[&str] = &[
    "groupBy",
    "noCache",
    "showSettings",
    "showIssueSeverity",
    "showIssueSourceLine",
    "showIssueRuleName",
    "showIssueDescription",
    "showSummary",
];
const ALLOWED_FILES: &[&str] = &["include", "exclude"];
const ALLOWED_CODE_EDITOR: &[&str] = &[
    "highlightErrors",
    "highlightWarnings",
    "scanIntervalSeconds",
];
const ALLOWED_CUSTOM_RULE_BASE: &[&str] = &[
    "type", "message", "severity", "enabled", "include", "exclude",
];
const ALLOWED_REGEX_RULE: &[&str] = &["pattern"];
const ALLOWED_SCRIPT_RULE: &[&str] = &["script", "runner", "mode", "timeout", "options"];
const ALLOWED_AI_RULE: &[&str] = &["prompt"];
const ALLOWED_BUILTIN_RULE_BASE: &[&str] = &["enabled", "severity", "include", "exclude"];

fn validate_builtin_rules(rules: &serde_json::Map<String, serde_json::Value>) -> Vec<String> {
    use crate::rules::get_allowed_options_for_rule;

    let mut invalid = Vec::new();
    for (rule_name, rule_config) in rules {
        if let Some(rule_obj) = rule_config.as_object() {
            let mut allowed: Vec<&str> = ALLOWED_BUILTIN_RULE_BASE.to_vec();
            allowed.extend(get_allowed_options_for_rule(rule_name));
            let prefix = format!("builtinRules.{}", rule_name);
            invalid.extend(collect_invalid_fields(rule_obj, &allowed, &prefix));
        }
    }
    invalid
}

fn collect_invalid_fields(
    obj: &serde_json::Map<String, serde_json::Value>,
    allowed: &[&str],
    prefix: &str,
) -> Vec<String> {
    let allowed_set: HashSet<&str> = allowed.iter().copied().collect();
    obj.keys()
        .filter(|key| !allowed_set.contains(key.as_str()))
        .map(|key| {
            if prefix.is_empty() {
                key.clone()
            } else {
                format!("{}.{}", prefix, key)
            }
        })
        .collect()
}

fn validate_custom_rules(rules: &serde_json::Map<String, serde_json::Value>) -> Vec<String> {
    let mut invalid = Vec::new();
    for (rule_name, rule_config) in rules {
        if let Some(rule_obj) = rule_config.as_object() {
            let prefix = format!("customRules.{}", rule_name);
            let rule_type = rule_obj.get("type").and_then(|v| v.as_str()).unwrap_or("");

            let mut allowed: Vec<&str> = ALLOWED_CUSTOM_RULE_BASE.to_vec();
            match rule_type {
                "regex" => allowed.extend(ALLOWED_REGEX_RULE),
                "script" => allowed.extend(ALLOWED_SCRIPT_RULE),
                "ai" => allowed.extend(ALLOWED_AI_RULE),
                _ => {}
            }

            invalid.extend(collect_invalid_fields(rule_obj, &allowed, &prefix));
        }
    }
    invalid
}

pub fn validate_json_fields(json: &serde_json::Value) -> ValidationResult {
    let mut result = ValidationResult::new();

    let Some(obj) = json.as_object() else {
        return result;
    };

    let mut invalid_fields = collect_invalid_fields(obj, ALLOWED_TOP_LEVEL, "");

    if let Some(files) = obj.get("files").and_then(|v| v.as_object()) {
        invalid_fields.extend(collect_invalid_fields(files, ALLOWED_FILES, "files"));
    }

    if let Some(code_editor) = obj.get("codeEditor").and_then(|v| v.as_object()) {
        invalid_fields.extend(collect_invalid_fields(
            code_editor,
            ALLOWED_CODE_EDITOR,
            "codeEditor",
        ));
    }

    if let Some(cli) = obj.get("cli").and_then(|v| v.as_object()) {
        invalid_fields.extend(collect_invalid_fields(cli, ALLOWED_CLI, "cli"));
    }

    if let Some(builtin_rules) = obj.get("builtinRules").and_then(|v| v.as_object()) {
        invalid_fields.extend(validate_builtin_rules(builtin_rules));
    }

    if let Some(custom_rules) = obj.get("customRules").and_then(|v| v.as_object()) {
        invalid_fields.extend(validate_custom_rules(custom_rules));
    }

    for field in invalid_fields {
        result.add_error(format!("Invalid field: {}", field));
    }

    result
}
