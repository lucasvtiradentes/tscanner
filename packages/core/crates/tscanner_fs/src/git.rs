use anyhow::{Context, Result};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::process::Command;

fn parse_modified_lines(diff_output: &str) -> HashMap<String, HashSet<usize>> {
    let mut file_lines: HashMap<String, HashSet<usize>> = HashMap::new();
    let mut current_file: Option<String> = None;
    let mut current_line: usize = 0;

    for line in diff_output.lines() {
        if line.starts_with("diff --git") {
            current_file = None;
            current_line = 0;
        } else if line.starts_with("+++") {
            if let Some(file_path) = line.strip_prefix("+++ b/") {
                current_file = Some(file_path.to_string());
            }
        } else if line.starts_with("@@") {
            if let Some(hunk_info) = line.split("@@").nth(1) {
                if let Some(new_info) = hunk_info.split_whitespace().nth(1) {
                    if let Some(line_num) = new_info.trim_start_matches('+').split(',').next() {
                        current_line = line_num.parse::<usize>().unwrap_or(0);
                    }
                }
            }
        } else if let Some(ref file) = current_file {
            if line.starts_with('+') && !line.starts_with("+++") {
                file_lines
                    .entry(file.clone())
                    .or_default()
                    .insert(current_line);
                current_line += 1;
            } else if !line.starts_with('-') {
                current_line += 1;
            }
        }
    }

    file_lines
}

pub fn get_changed_files(root: &Path, branch: &str) -> Result<HashSet<PathBuf>> {
    let output = Command::new("git")
        .args(["diff", "-w", "--name-only", branch])
        .current_dir(root)
        .output()
        .context("Failed to execute git diff")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("git diff failed: {}", stderr);
    }

    let files = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|line| root.join(line.trim()))
        .collect();

    Ok(files)
}

pub fn get_staged_files(root: &Path) -> Result<HashSet<PathBuf>> {
    let output = Command::new("git")
        .args([
            "diff",
            "-w",
            "--cached",
            "--name-only",
            "--diff-filter=ACMR",
        ])
        .current_dir(root)
        .output()
        .context("Failed to execute git diff --cached")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("git diff --cached failed: {}", stderr);
    }

    let files = String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| root.join(l))
        .collect();

    Ok(files)
}

pub fn get_modified_lines(root: &Path, branch: &str) -> Result<HashMap<PathBuf, HashSet<usize>>> {
    let output = Command::new("git")
        .args(["diff", "-w", branch])
        .current_dir(root)
        .output()
        .context("Failed to execute git diff")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("git diff failed: {}", stderr);
    }

    let diff_text = String::from_utf8_lossy(&output.stdout);
    let file_lines = parse_modified_lines(&diff_text);

    let result = file_lines
        .into_iter()
        .map(|(file, lines)| (root.join(file), lines))
        .collect();

    Ok(result)
}

pub fn get_staged_modified_lines(root: &Path) -> Result<HashMap<PathBuf, HashSet<usize>>> {
    let output = Command::new("git")
        .args(["diff", "-w", "--cached"])
        .current_dir(root)
        .output()
        .context("Failed to execute git diff --cached")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("git diff --cached failed: {}", stderr);
    }

    let diff_text = String::from_utf8_lossy(&output.stdout);
    let file_lines = parse_modified_lines(&diff_text);

    let result = file_lines
        .into_iter()
        .map(|(file, lines)| (root.join(file), lines))
        .collect();

    Ok(result)
}
