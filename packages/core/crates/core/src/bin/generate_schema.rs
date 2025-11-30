use core::config::TscannerConfig;
use core::rules::get_all_rule_metadata;
use schemars::schema_for;
use serde_json::{json, Map, Value};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

fn get_rule_options_schema() -> HashMap<&'static str, Value> {
    let mut options = HashMap::new();

    options.insert(
        "max-params",
        json!({
            "maxParams": {
                "type": "integer",
                "default": 4,
                "minimum": 1,
                "description": "Maximum number of parameters allowed in a function"
            }
        }),
    );

    options.insert(
        "max-function-length",
        json!({
            "maxLength": {
                "type": "integer",
                "default": 50,
                "minimum": 1,
                "description": "Maximum number of statements allowed in a function"
            }
        }),
    );

    options.insert(
        "no-todo-comments",
        json!({
            "keywords": {
                "type": "array",
                "items": { "type": "string" },
                "default": ["TODO", "FIXME", "HACK", "XXX", "NOTE", "BUG"],
                "description": "Comment keywords to detect"
            }
        }),
    );

    options.insert(
        "no-console",
        json!({
            "methods": {
                "type": "array",
                "items": { "type": "string" },
                "default": ["log", "warn", "error", "info", "debug", "trace", "table", "dir", "dirxml", "group", "groupCollapsed", "groupEnd", "time", "timeEnd", "timeLog", "assert", "count", "countReset", "clear", "profile", "profileEnd"],
                "description": "Console methods to disallow"
            }
        }),
    );

    options
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schema = schema_for!(TscannerConfig);
    let mut schema_value: Value = serde_json::to_value(&schema)?;

    let metadata = get_all_rule_metadata();
    let rule_options = get_rule_options_schema();

    let base_rule_props = json!({
        "enabled": {
            "description": "Enable or disable this rule",
            "type": ["boolean", "null"]
        },
        "severity": {
            "anyOf": [
                { "$ref": "#/definitions/Severity" },
                { "type": "null" }
            ],
            "description": "Severity level for this rule"
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

        if let Some(options) = rule_options.get(meta.name) {
            if let Some(opts_obj) = options.as_object() {
                for (key, value) in opts_obj {
                    properties.insert(key.clone(), value.clone());
                }
            }
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

    let mut rule_properties = Map::new();
    for meta in &metadata {
        rule_properties.insert(
            meta.name.to_string(),
            json!({
                "$ref": format!("#/definitions/BuiltinRuleConfig_{}", meta.name.replace('-', "_"))
            }),
        );
    }

    if let Some(properties) = schema_value
        .get_mut("properties")
        .and_then(|p| p.as_object_mut())
    {
        if let Some(builtin_rules) = properties.get_mut("builtinRules") {
            if let Some(obj) = builtin_rules.as_object_mut() {
                obj.insert("properties".to_string(), Value::Object(rule_properties));
                obj.insert("additionalProperties".to_string(), json!(false));
            }
        }
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
