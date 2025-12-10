use serde_json::Value;
use std::collections::{HashMap, HashSet};

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

const SCHEMA_JSON: &str = include_str!("../../../../cli/schema.json");

struct SchemaFields {
    tscanner_config: Vec<String>,
    ai_config: Vec<String>,
    code_editor_config: Vec<String>,
    files_config: Vec<String>,
    rules_config: Vec<String>,
    regex_rule_config: Vec<String>,
    script_rule_config: Vec<String>,
    ai_rule_config: Vec<String>,
    builtin_rule_base: Vec<String>,
    builtin_rule_options: HashMap<String, Vec<String>>,
}

fn extract_properties(schema: &Value, path: &str) -> Vec<String> {
    let parts: Vec<&str> = path.split('.').collect();
    let mut current = schema;

    for part in parts {
        current = match current.get(part) {
            Some(v) => v,
            None => return vec![],
        };
    }

    current
        .as_object()
        .map(|obj| obj.keys().cloned().collect())
        .unwrap_or_default()
}

fn extract_definition_properties(schema: &Value, definition_name: &str) -> Vec<String> {
    extract_properties(
        schema,
        &format!("definitions.{}.properties", definition_name),
    )
}

fn build_schema_fields(schema: &Value) -> SchemaFields {
    let mut tscanner_config = extract_properties(schema, "properties");
    tscanner_config.push("$schema".to_string());

    let builtin_rule_base = extract_definition_properties(schema, "BuiltinRuleConfig");

    let mut builtin_rule_options: HashMap<String, Vec<String>> = HashMap::new();
    if let Some(definitions) = schema.get("definitions").and_then(|d| d.as_object()) {
        for (name, _) in definitions {
            if name.starts_with("BuiltinRuleConfig_") {
                let rule_name = name
                    .strip_prefix("BuiltinRuleConfig_")
                    .unwrap()
                    .replace('_', "-");
                let props = extract_definition_properties(schema, name);
                let extra: Vec<String> = props
                    .into_iter()
                    .filter(|p| !builtin_rule_base.contains(p))
                    .collect();
                if !extra.is_empty() {
                    builtin_rule_options.insert(rule_name, extra);
                }
            }
        }
    }

    SchemaFields {
        tscanner_config,
        ai_config: extract_definition_properties(schema, "AiConfig"),
        code_editor_config: extract_definition_properties(schema, "CodeEditorConfig"),
        files_config: extract_definition_properties(schema, "FilesConfig"),
        rules_config: extract_definition_properties(schema, "RulesConfig"),
        regex_rule_config: extract_definition_properties(schema, "RegexRuleConfig"),
        script_rule_config: extract_definition_properties(schema, "ScriptRuleConfig"),
        ai_rule_config: extract_definition_properties(schema, "AiRuleConfig"),
        builtin_rule_base,
        builtin_rule_options,
    }
}

lazy_static::lazy_static! {
    static ref SCHEMA: Value = serde_json::from_str(SCHEMA_JSON)
        .expect("Failed to parse schema.json");
    static ref FIELDS: SchemaFields = build_schema_fields(&SCHEMA);
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

fn validate_builtin_rules(
    rules: &serde_json::Map<String, serde_json::Value>,
    prefix: &str,
) -> Vec<String> {
    let mut invalid = Vec::new();
    for (rule_name, rule_config) in rules {
        if let Some(rule_obj) = rule_config.as_object() {
            let mut allowed = FIELDS.builtin_rule_base.clone();
            if let Some(extra) = FIELDS.builtin_rule_options.get(rule_name) {
                allowed.extend(extra.clone());
            }
            let rule_prefix = format!("{}.{}", prefix, rule_name);
            invalid.extend(collect_invalid_fields(rule_obj, &allowed, &rule_prefix));
        }
    }
    invalid
}

fn validate_custom_rules(
    rules: &serde_json::Map<String, serde_json::Value>,
    allowed_fields: &[String],
    prefix_format: &str,
) -> Vec<String> {
    let mut invalid = Vec::new();
    for (rule_name, rule_config) in rules {
        if let Some(rule_obj) = rule_config.as_object() {
            let prefix = format!("{}.{}", prefix_format, rule_name);
            invalid.extend(collect_invalid_fields(rule_obj, allowed_fields, &prefix));
        }
    }
    invalid
}

pub fn validate_json_fields(json: &serde_json::Value) -> ValidationResult {
    let mut result = ValidationResult::new();

    let Some(obj) = json.as_object() else {
        return result;
    };

    let mut invalid_fields = collect_invalid_fields(obj, &FIELDS.tscanner_config, "");

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

    if let Some(ai) = obj.get("ai").and_then(|v| v.as_object()) {
        invalid_fields.extend(collect_invalid_fields(ai, &FIELDS.ai_config, "ai"));
    }

    if let Some(rules) = obj.get("rules").and_then(|v| v.as_object()) {
        invalid_fields.extend(collect_invalid_fields(rules, &FIELDS.rules_config, "rules"));

        if let Some(builtin_rules) = rules.get("builtin").and_then(|v| v.as_object()) {
            invalid_fields.extend(validate_builtin_rules(builtin_rules, "rules.builtin"));
        }

        if let Some(regex_rules) = rules.get("regex").and_then(|v| v.as_object()) {
            invalid_fields.extend(validate_custom_rules(
                regex_rules,
                &FIELDS.regex_rule_config,
                "rules.regex",
            ));
        }

        if let Some(script_rules) = rules.get("script").and_then(|v| v.as_object()) {
            invalid_fields.extend(validate_custom_rules(
                script_rules,
                &FIELDS.script_rule_config,
                "rules.script",
            ));
        }
    }

    if let Some(ai_rules) = obj.get("aiRules").and_then(|v| v.as_object()) {
        invalid_fields.extend(validate_custom_rules(
            ai_rules,
            &FIELDS.ai_rule_config,
            "aiRules",
        ));
    }

    for field in invalid_fields {
        result.add_error(format!("Invalid field: {}", field));
    }

    result
}
