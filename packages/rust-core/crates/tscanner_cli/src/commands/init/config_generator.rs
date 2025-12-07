use serde_json::Value;
use tscanner_rules::get_all_rule_metadata;

const DEFAULT_CONFIG_JSON: &str = include_str!("../../../../../../../assets/default-config.json");
const TSCANNER_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn get_default_config() -> String {
    let mut config: Value = serde_json::from_str(DEFAULT_CONFIG_JSON).unwrap_or_default();

    if let Some(obj) = config.as_object_mut() {
        obj["$schema"] = Value::String(format!(
            "https://unpkg.com/tscanner@{}/schema.json",
            TSCANNER_VERSION
        ));
    }

    serde_json::to_string_pretty(&config).unwrap_or_else(|_| DEFAULT_CONFIG_JSON.to_string())
}

pub fn get_all_rules_config() -> String {
    let mut config: Value = serde_json::from_str(DEFAULT_CONFIG_JSON).unwrap_or_default();

    if let Some(obj) = config.as_object_mut() {
        obj["$schema"] = Value::String(format!(
            "https://unpkg.com/tscanner@{}/schema.json",
            TSCANNER_VERSION
        ));

        let metadata = get_all_rule_metadata();
        let mut rule_names: Vec<&str> = metadata.iter().map(|m| m.name).collect();
        rule_names.sort();

        let mut builtin_rules = serde_json::Map::new();
        for name in rule_names {
            builtin_rules.insert(name.to_string(), Value::Object(serde_json::Map::new()));
        }

        if let Some(rules) = obj.get_mut("rules").and_then(|r| r.as_object_mut()) {
            rules["builtin"] = Value::Object(builtin_rules);
        } else {
            let mut rules = serde_json::Map::new();
            rules.insert("builtin".to_string(), Value::Object(builtin_rules));
            obj["rules"] = Value::Object(rules);
        }
    }

    serde_json::to_string_pretty(&config).unwrap_or_else(|_| DEFAULT_CONFIG_JSON.to_string())
}
