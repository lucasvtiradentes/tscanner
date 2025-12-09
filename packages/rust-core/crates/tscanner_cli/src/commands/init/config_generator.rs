use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

const TSCANNER_VERSION: &str = env!("CARGO_PKG_VERSION");

const MINIMAL_CONFIG_JSON: &str = include_str!("../../../../../../../assets/configs/minimal.json");
const FULL_CONFIG_JSON: &str = include_str!("../../../../../../../assets/configs/full.json");
const EXAMPLE_SCRIPT: &str =
    include_str!("../../../../../../../assets/configs/script-rule-example.ts");
const EXAMPLE_PROMPT: &str = include_str!("../../../../../../../assets/configs/ai-rule-example.md");
const DEV_SCHEMA: &str = "\"$schema\": \"../../packages/cli/schema.json\"";

fn process_config(config_json: &str) -> String {
    let prod_schema = format!(
        "\"$schema\": \"https://unpkg.com/tscanner@{}/schema.json\"",
        TSCANNER_VERSION
    );
    config_json.replace(DEV_SCHEMA, &prod_schema)
}

pub fn get_default_config() -> String {
    process_config(MINIMAL_CONFIG_JSON)
}

pub fn get_full_config() -> String {
    process_config(FULL_CONFIG_JSON)
}

pub fn write_example_files(config_dir: &Path) -> Result<()> {
    let script_rules_dir = config_dir.join("script-rules");
    fs::create_dir_all(&script_rules_dir).context("Failed to create script-rules directory")?;
    fs::write(
        script_rules_dir.join("example-no-debug-comments.ts"),
        EXAMPLE_SCRIPT,
    )
    .context("Failed to write example script")?;

    let ai_rules_dir = config_dir.join("ai-rules");
    fs::create_dir_all(&ai_rules_dir).context("Failed to create ai-rules directory")?;
    fs::write(
        ai_rules_dir.join("example-find-complexity.md"),
        EXAMPLE_PROMPT,
    )
    .context("Failed to write example prompt")?;

    Ok(())
}
