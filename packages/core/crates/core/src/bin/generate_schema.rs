use core::config::TscannerConfig;
use core::rules::get_all_rule_metadata;
use schemars::schema_for;
use serde_json::{json, Map, Value};
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schema = schema_for!(TscannerConfig);
    let mut schema_value: Value = serde_json::to_value(&schema)?;

    let metadata = get_all_rule_metadata();
    let mut rule_properties = Map::new();

    for meta in &metadata {
        rule_properties.insert(
            meta.name.to_string(),
            json!({
                "description": meta.description,
                "$ref": "#/definitions/BuiltinRuleConfig"
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
