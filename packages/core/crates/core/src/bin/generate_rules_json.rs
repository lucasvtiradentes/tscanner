use core::rules::get_all_rule_metadata;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rules = get_all_rule_metadata();
    rules.sort_by(|a, b| a.name.cmp(b.name));

    let json = serde_json::to_string_pretty(&rules)?;

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
        .join("assets/rules.json");

    fs::write(&output_path, json)?;

    println!("✓ Generated rules.json at: {}", output_path.display());
    println!("  Total rules: {}", rules.len());
    Ok(())
}
