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
    let scripts_dir = config_dir.join("scripts");
    fs::create_dir_all(&scripts_dir).context("Failed to create scripts directory")?;
    fs::write(
        scripts_dir.join("example-no-debug-comments.ts"),
        EXAMPLE_SCRIPT,
    )
    .context("Failed to write example script")?;

    let prompts_dir = config_dir.join("prompts");
    fs::create_dir_all(&prompts_dir).context("Failed to create prompts directory")?;
    fs::write(
        prompts_dir.join("example-find-complexity.md"),
        EXAMPLE_PROMPT,
    )
    .context("Failed to write example prompt")?;

    Ok(())
}
