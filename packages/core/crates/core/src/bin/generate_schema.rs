use core::config::TscannerConfig;
use schemars::schema_for;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schema = schema_for!(TscannerConfig);
    let json = serde_json::to_string_pretty(&schema)?;

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

    println!("✓ Generated schema.json at: {}", output_path.display());
    Ok(())
}
