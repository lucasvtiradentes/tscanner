use crate::scanner::Scanner;
use ignore::WalkBuilder;
use std::collections::HashSet;
use std::path::PathBuf;

impl Scanner {
    pub(crate) fn collect_files_with_filter(
        &self,
        roots: &[PathBuf],
        file_filter: Option<&HashSet<PathBuf>>,
    ) -> Vec<PathBuf> {
        let mut files: Vec<PathBuf> = Vec::new();

        for root in roots {
            let root_buf = root.to_path_buf();
            let exclude_clone = self.global_exclude.clone();
            let root_clone = root_buf.clone();
            let include_clone = self.global_include.clone();
            let custom_include_clone = self.custom_include.clone();
            let exclude_clone2 = self.global_exclude.clone();

            let root_files: Vec<PathBuf> = WalkBuilder::new(root)
                .hidden(false)
                .git_ignore(true)
                .filter_entry(move |e| {
                    let path = e.path();
                    if path.is_dir() {
                        let relative = path.strip_prefix(&root_clone).unwrap_or(path);
                        return !exclude_clone.is_match(relative);
                    }
                    true
                })
                .build()
                .filter_map(|e| e.ok())
                .filter(|e| {
                    let path = e.path();
                    if !path.is_file() {
                        return false;
                    }
                    let relative = path.strip_prefix(&root_buf).unwrap_or(path);
                    if exclude_clone2.is_match(relative) {
                        return false;
                    }
                    let matches_global = include_clone.is_match(relative);
                    let matches_custom = custom_include_clone
                        .as_ref()
                        .map(|g| g.is_match(relative))
                        .unwrap_or(false);
                    matches_global || matches_custom
                })
                .map(|e| e.path().to_path_buf())
                .collect();

            files.extend(root_files);
        }

        if let Some(filter) = file_filter {
            files.retain(|f| filter.contains(f));
            (self.log_info)(&format!(
                "Filtered to {} files (from {})",
                files.len(),
                filter.len()
            ));
        }

        files
    }
}
