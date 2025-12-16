#!/usr/bin/env rust-script

use std::io::{self, Read};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct ScriptFile {
    path: String,
    lines: Vec<String>,
}

#[derive(Deserialize)]
struct ScriptInput {
    files: Vec<ScriptFile>,
}

#[derive(Serialize)]
struct ScriptIssue {
    file: String,
    line: usize,
    message: String,
}

#[derive(Serialize)]
struct ScriptOutput {
    issues: Vec<ScriptIssue>,
}

fn main() -> io::Result<()> {
    let mut data = String::new();
    io::stdin().read_to_string(&mut data)?;

    let input: ScriptInput = serde_json::from_str(&data).expect("Invalid JSON input");
    let mut issues = Vec::new();

    for file in input.files {
        for (idx, line) in file.lines.iter().enumerate() {
            let upper = line.to_uppercase();
            if upper.contains("FIXME") || upper.contains("XXX") {
                issues.push(ScriptIssue {
                    file: file.path.clone(),
                    line: idx + 1,
                    message: "FIXME/XXX comment found".to_string(),
                });
            }
        }
    }

    let output = ScriptOutput { issues };
    println!("{}", serde_json::to_string(&output).unwrap());
    Ok(())
}
