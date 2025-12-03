use serde::Serialize;
use serde_json::json;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize)]
struct ConfigFields {
    tscanner_config: Vec<&'static str>,
    code_editor_config: Vec<&'static str>,
    cli_config: Vec<&'static str>,
    files_config: Vec<&'static str>,
    builtin_rule_config: Vec<&'static str>,
    custom_rule_base: Vec<&'static str>,
    regex_rule_config: Vec<&'static str>,
    script_rule_config: Vec<&'static str>,
    ai_rule_config: Vec<&'static str>,
    script_mode: Vec<&'static str>,
    custom_rule_types: Vec<&'static str>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fields = ConfigFields {
        tscanner_config: vec!["codeEditor", "cli", "builtinRules", "customRules", "files"],
        code_editor_config: vec![
            "highlightErrors",
            "highlightWarnings",
            "scanIntervalSeconds",
        ],
        cli_config: vec![
            "groupBy",
            "noCache",
            "showSettings",
            "showIssueSeverity",
            "showIssueSourceLine",
            "showIssueRuleName",
            "showIssueDescription",
            "showSummary",
        ],
        files_config: vec!["include", "exclude"],
        builtin_rule_config: vec!["enabled", "severity", "include", "exclude"],
        custom_rule_base: vec!["message", "severity", "enabled", "include", "exclude"],
        regex_rule_config: vec!["type", "pattern"],
        script_rule_config: vec!["type", "command", "mode", "timeout", "options"],
        ai_rule_config: vec!["type", "prompt"],
        script_mode: vec!["batch", "single"],
        custom_rule_types: vec!["regex", "script", "ai"],
    };

    let json = serde_json::to_string_pretty(&json!(fields))?;

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
        .join("assets/generated/rust_config_fields.json");

    fs::write(&output_path, &json)?;

    println!(
        "✓ Generated rust_config_fields.json at: {}",
        output_path.display()
    );

    Ok(())
}
