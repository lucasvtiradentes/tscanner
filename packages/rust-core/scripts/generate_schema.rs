use schemars::schema_for;
use serde_json::{json, Map, Value};
use std::fs;
use std::path::PathBuf;
use tscanner_config::TscannerConfig;
use tscanner_rules::{get_all_rule_metadata, RuleOptionSchema};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schema = schema_for!(TscannerConfig);
    let mut schema_value: Value = serde_json::to_value(&schema)?;

    let metadata = get_all_rule_metadata();

    let base_rule_props = json!({
        "severity": {
            "allOf": [
                { "$ref": "#/definitions/Severity" }
            ],
            "description": "Severity level for this rule (default: warning)"
        },
        "include": {
            "description": "File patterns to include for this rule",
            "items": { "type": "string" },
            "type": "array"
        },
        "exclude": {
            "description": "File patterns to exclude for this rule",
            "items": { "type": "string" },
            "type": "array"
        }
    });

    let mut rule_definitions = Map::new();

    for meta in &metadata {
        let mut rule_schema = Map::new();
        rule_schema.insert("type".to_string(), json!("object"));
        rule_schema.insert("description".to_string(), json!(meta.description));

        let mut properties = base_rule_props.as_object().unwrap().clone();

        for opt in meta.options {
            let opt_schema = match &opt.schema {
                RuleOptionSchema::Integer { default, minimum } => {
                    let mut schema = json!({
                        "type": "integer",
                        "default": default,
                        "description": opt.description
                    });
                    if let Some(min) = minimum {
                        schema["minimum"] = json!(min);
                    }
                    schema
                }
                RuleOptionSchema::Boolean { default } => {
                    json!({
                        "type": "boolean",
                        "default": default,
                        "description": opt.description
                    })
                }
                RuleOptionSchema::String { default } => {
                    json!({
                        "type": "string",
                        "default": default,
                        "description": opt.description
                    })
                }
                RuleOptionSchema::Array { items, default } => {
                    json!({
                        "type": "array",
                        "items": { "type": items },
                        "default": default,
                        "description": opt.description
                    })
                }
            };
            properties.insert(opt.name.to_string(), opt_schema);
        }

        rule_schema.insert("properties".to_string(), Value::Object(properties));

        rule_definitions.insert(
            format!("BuiltinRuleConfig_{}", meta.name.replace('-', "_")),
            Value::Object(rule_schema),
        );
    }

    if let Some(definitions) = schema_value.get_mut("definitions") {
        if let Some(defs_obj) = definitions.as_object_mut() {
            for (key, value) in rule_definitions {
                defs_obj.insert(key, value);
            }
        }
    }

    let mut builtin_rule_properties = Map::new();
    for meta in &metadata {
        builtin_rule_properties.insert(
            meta.name.to_string(),
            json!({
                "$ref": format!("#/definitions/BuiltinRuleConfig_{}", meta.name.replace('-', "_"))
            }),
        );
    }

    if let Some(definitions) = schema_value.get_mut("definitions") {
        if let Some(defs_obj) = definitions.as_object_mut() {
            if let Some(rules_config) = defs_obj.get_mut("RulesConfig") {
                if let Some(rules_config_obj) = rules_config.as_object_mut() {
                    if let Some(rules_props) = rules_config_obj.get_mut("properties") {
                        if let Some(rules_props_obj) = rules_props.as_object_mut() {
                            rules_props_obj.insert(
                                "builtin".to_string(),
                                json!({
                                    "type": "object",
                                    "description": "Built-in AST rules",
                                    "properties": builtin_rule_properties,
                                    "additionalProperties": false
                                }),
                            );

                            rules_props_obj.insert(
                                "regex".to_string(),
                                json!({
                                    "type": "object",
                                    "description": "Custom regex pattern rules",
                                    "additionalProperties": { "$ref": "#/definitions/RegexRuleConfig" }
                                }),
                            );

                            rules_props_obj.insert(
                                "script".to_string(),
                                json!({
                                    "type": "object",
                                    "description": "Custom script rules",
                                    "additionalProperties": { "$ref": "#/definitions/ScriptRuleConfig" }
                                }),
                            );
                        }
                    }
                }
            }
        }
    }

    if let Some(properties) = schema_value
        .get_mut("properties")
        .and_then(|p| p.as_object_mut())
    {
        properties.insert(
            "aiRules".to_string(),
            json!({
                "type": "object",
                "description": "AI-powered rules (expensive, run separately)",
                "additionalProperties": { "$ref": "#/definitions/AiRuleConfig" }
            }),
        );
    }

    if let Some(definitions) = schema_value
        .get_mut("definitions")
        .and_then(|d| d.as_object_mut())
    {
        definitions.insert(
            "AiConfig".to_string(),
            json!({
                "type": "object",
                "properties": {
                    "provider": {
                        "anyOf": [
                            { "$ref": "#/definitions/AiProvider" },
                            { "type": "null" }
                        ],
                        "description": "AI provider to use (claude, gemini, custom)"
                    },
                    "command": {
                        "type": ["string", "null"],
                        "description": "Custom command path (required when provider is 'custom')"
                    }
                },
                "if": {
                    "properties": {
                        "provider": { "const": "custom" }
                    },
                    "required": ["provider"]
                },
                "then": {
                    "required": ["command"],
                    "properties": {
                        "command": {
                            "type": "string",
                            "minLength": 1
                        }
                    }
                }
            }),
        );
    }

    let json = serde_json::to_string_pretty(&schema_value)?;

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let output_path = PathBuf::from(manifest_dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("cli/schema.json");

    fs::write(&output_path, json)?;

    println!(
        "✓ Generated schema.json with {} builtin rules at: {}",
        metadata.len(),
        output_path.display()
    );
    Ok(())
}
