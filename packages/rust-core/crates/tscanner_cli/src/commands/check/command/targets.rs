use anyhow::{Context, Result};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use crate::shared::ScanMode;
use tscanner_service::log_info;

use super::ModifiedLinesMap;
use crate::commands::check::{filters, git};
use crate::shared::fatal_error_and_exit;

pub(super) fn resolve_scan_paths(
    root: &Path,
    paths: &[PathBuf],
    staged: bool,
    uncommitted: bool,
) -> Result<Vec<PathBuf>> {
    if staged || uncommitted {
        return Ok(vec![root.to_path_buf()]);
    }

    paths
        .iter()
        .map(|p| fs::canonicalize(p).context(format!("Failed to resolve path: {}", p.display())))
        .collect::<Result<Vec<_>>>()
}

fn get_branch_changes(
    root: &Path,
    branch_name: &str,
) -> Result<(Option<HashSet<PathBuf>>, Option<ModifiedLinesMap>)> {
    match (
        git::get_changed_files(root, branch_name),
        git::get_modified_lines(root, branch_name),
    ) {
        (Ok(files), Ok(lines)) => {
            log_info(&format!(
                "cmd_check: Found {} changed files vs {}",
                files.len(),
                branch_name
            ));
            Ok((Some(files), Some(lines)))
        }
        (Err(e), _) | (_, Err(e)) => {
            fatal_error_and_exit(&format!("Error getting changed files: {}", e), &[]);
        }
    }
}

pub(super) fn resolve_scan_targets(
    root: &Path,
    scan_paths: &[PathBuf],
    glob_filter: Option<&str>,
    branch: Option<&String>,
    staged: bool,
    uncommitted: bool,
) -> Result<(Option<HashSet<PathBuf>>, Option<ModifiedLinesMap>, ScanMode)> {
    if staged {
        let staged_files = git::get_staged_files(root)?;
        let staged_lines = git::get_staged_modified_lines(root)?;
        let file_count = staged_files.len();
        log_info(&format!("cmd_check: Found {} staged files", file_count));
        let files = filters::get_files_to_scan_multi(scan_paths, glob_filter, Some(staged_files));
        return Ok((files, Some(staged_lines), ScanMode::Staged { file_count }));
    }

    if uncommitted {
        let uncommitted_files = git::get_uncommitted_files(root)?;
        let uncommitted_lines = git::get_uncommitted_modified_lines(root)?;
        let file_count = uncommitted_files.len();
        log_info(&format!(
            "cmd_check: Found {} uncommitted files",
            file_count
        ));
        let files =
            filters::get_files_to_scan_multi(scan_paths, glob_filter, Some(uncommitted_files));
        return Ok((
            files,
            Some(uncommitted_lines),
            ScanMode::Uncommitted { file_count },
        ));
    }

    if let Some(branch_name) = branch {
        let (changed_files, modified_lines) = get_branch_changes(root, branch_name)?;
        let file_count = changed_files.as_ref().map_or(0, |f| f.len());
        let files = filters::get_files_to_scan_multi(scan_paths, glob_filter, changed_files);
        return Ok((
            files,
            modified_lines,
            ScanMode::Branch {
                name: branch_name.clone(),
                file_count,
            },
        ));
    }

    let files = filters::get_files_to_scan_multi(scan_paths, glob_filter, None);
    Ok((files, None, ScanMode::Codebase))
}
