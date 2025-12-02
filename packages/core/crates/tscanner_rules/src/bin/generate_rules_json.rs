use std::fs;
use std::path::PathBuf;
use tscanner_rules::{get_all_rule_metadata, RuleCategory};

fn category_to_path(category: &RuleCategory) -> &'static str {
    match category {
        RuleCategory::BugPrevention => "bug_prevention",
        RuleCategory::CodeQuality => "code_quality",
        RuleCategory::TypeSafety => "type_safety",
        RuleCategory::Style => "style",
        RuleCategory::Performance => "performance",
        RuleCategory::Variables => "variables",
        RuleCategory::Imports => "imports",
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rules = get_all_rule_metadata();
    rules.sort_by(|a, b| a.name.cmp(b.name));

    let output: Vec<serde_json::Value> = rules
        .iter()
        .map(|r| {
            let mut obj = serde_json::to_value(r).unwrap();
            let map = obj.as_object_mut().unwrap();

            let category_path = category_to_path(&r.category);
            let snake_name = r.name.replace('-', "_");
            map.insert(
                "sourcePath".to_string(),
                serde_json::Value::String(format!(
                    "packages/core/crates/tscanner_rules/src/builtin/{}/{}.rs",
                    category_path, snake_name
                )),
            );

            obj
        })
        .collect();

    let json = serde_json::to_string_pretty(&output)?;

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let output_path = PathBuf::from(manifest_dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("assets/generated/rules.json");

    fs::write(&output_path, json)?;

    println!("✓ Generated rules.json at: {}", output_path.display());
    println!("  Total rules: {}", rules.len());
    Ok(())
}
