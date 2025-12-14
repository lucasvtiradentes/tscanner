use anyhow::{anyhow, Result};
use colored::*;
use std::path::PathBuf;
use tscanner_constants::{config_dir_name, config_file_name};
use tscanner_scanner::load_config;

use crate::shared::{print_section_header, render_warnings};

pub fn validate(config_path: Option<PathBuf>) -> Result<()> {
    let path = config_path.unwrap_or_else(|| PathBuf::from("."));

    let (_config, warnings) =
        load_config(&path, config_dir_name(), config_file_name()).map_err(|e| anyhow!("{}", e))?;

    render_warnings(&warnings);

    if !warnings.is_empty() {
        println!();
    }

    print_section_header("Result:");
    println!("  {} {}", "✓".green(), "Config is valid".green());
    println!();

    Ok(())
}
