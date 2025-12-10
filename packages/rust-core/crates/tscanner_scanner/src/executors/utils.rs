use std::path::Path;

pub fn file_matches_patterns(
    path: &Path,
    workspace_root: &Path,
    include: &[String],
    exclude: &[String],
) -> bool {
    let relative = path.strip_prefix(workspace_root).unwrap_or(path);
    let relative_str = relative.to_string_lossy();

    if !include.is_empty()
        && !include
            .iter()
            .any(|pattern| glob_match::glob_match(pattern, &relative_str))
    {
        return false;
    }

    if exclude
        .iter()
        .any(|pattern| glob_match::glob_match(pattern, &relative_str))
    {
        return false;
    }

    true
}

pub fn extract_line_text(lines: &[&str], line_num: usize) -> Option<String> {
    if line_num > 0 && line_num <= lines.len() {
        Some(lines[line_num - 1].to_string())
    } else {
        None
    }
}
