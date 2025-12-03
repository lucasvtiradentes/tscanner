use serde::Deserialize;
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

const CONFIG_FIELDS_JSON: &str =
    include_str!("../../../../../assets/generated/rust_config_fields.json");

#[derive(Deserialize)]
struct ConfigFields {
    tscanner_config: Vec<String>,
    code_editor_config: Vec<String>,
    cli_config: Vec<String>,
    files_config: Vec<String>,
    builtin_rule_config: Vec<String>,
    custom_rule_base: Vec<String>,
    regex_rule_config: Vec<String>,
    script_rule_config: Vec<String>,
    ai_rule_config: Vec<String>,
}

lazy_static::lazy_static! {
    static ref FIELDS: ConfigFields = serde_json::from_str(CONFIG_FIELDS_JSON)
        .expect("Failed to parse rust_config_fields.json");
}

fn collect_invalid_fields(
    obj: &serde_json::Map<String, serde_json::Value>,
    allowed: &[String],
    prefix: &str,
) -> Vec<String> {
    let allowed_set: HashSet<&str> = allowed.iter().map(|s| s.as_str()).collect();
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

            let mut allowed: Vec<String> = FIELDS.custom_rule_base.clone();
            match rule_type {
                "regex" => allowed.extend(FIELDS.regex_rule_config.clone()),
                "script" => allowed.extend(FIELDS.script_rule_config.clone()),
                "ai" => allowed.extend(FIELDS.ai_rule_config.clone()),
                _ => {}
            }

            invalid.extend(collect_invalid_fields(rule_obj, &allowed, &prefix));
        }
    }
    invalid
}

pub type AllowedOptionsGetter = fn(&str) -> &'static [&'static str];

fn get_allowed_top_level() -> Vec<String> {
    let mut allowed = FIELDS.tscanner_config.clone();
    allowed.push("$schema".to_string());
    allowed
}

pub fn validate_json_fields(
    json: &serde_json::Value,
    get_allowed_options_for_rule: Option<AllowedOptionsGetter>,
) -> ValidationResult {
    let mut result = ValidationResult::new();

    let Some(obj) = json.as_object() else {
        return result;
    };

    let mut invalid_fields = collect_invalid_fields(obj, &get_allowed_top_level(), "");

    if let Some(files) = obj.get("files").and_then(|v| v.as_object()) {
        invalid_fields.extend(collect_invalid_fields(files, &FIELDS.files_config, "files"));
    }

    if let Some(code_editor) = obj.get("codeEditor").and_then(|v| v.as_object()) {
        invalid_fields.extend(collect_invalid_fields(
            code_editor,
            &FIELDS.code_editor_config,
            "codeEditor",
        ));
    }

    if let Some(cli) = obj.get("cli").and_then(|v| v.as_object()) {
        invalid_fields.extend(collect_invalid_fields(cli, &FIELDS.cli_config, "cli"));
    }

    if let Some(builtin_rules) = obj.get("builtinRules").and_then(|v| v.as_object()) {
        invalid_fields.extend(validate_builtin_rules(
            builtin_rules,
            get_allowed_options_for_rule,
        ));
    }

    if let Some(custom_rules) = obj.get("customRules").and_then(|v| v.as_object()) {
        invalid_fields.extend(validate_custom_rules(custom_rules));
    }

    for field in invalid_fields {
        result.add_error(format!("Invalid field: {}", field));
    }

    result
}

fn validate_builtin_rules(
    rules: &serde_json::Map<String, serde_json::Value>,
    get_allowed_options_for_rule: Option<AllowedOptionsGetter>,
) -> Vec<String> {
    let mut invalid = Vec::new();
    for (rule_name, rule_config) in rules {
        if let Some(rule_obj) = rule_config.as_object() {
            let mut allowed: Vec<String> = FIELDS.builtin_rule_config.clone();
            if let Some(getter) = get_allowed_options_for_rule {
                allowed.extend(getter(rule_name).iter().map(|s| s.to_string()));
            }
            let prefix = format!("builtinRules.{}", rule_name);
            invalid.extend(collect_invalid_fields(rule_obj, &allowed, &prefix));
        }
    }
    invalid
}
