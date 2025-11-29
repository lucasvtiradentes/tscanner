use globset::{Glob, GlobSetBuilder};
use std::path::Path;

fn compile_globset(patterns: &[&str]) -> globset::GlobSet {
    let mut builder = GlobSetBuilder::new();
    for pattern in patterns {
        let glob = Glob::new(pattern).expect("Invalid glob pattern");
        builder.add(glob);
    }
    builder.build().expect("Failed to build GlobSet")
}

#[test]
fn test_basic_pattern_matching() {
    let globset = compile_globset(&["**/*.ts"]);

    assert!(globset.is_match(Path::new("src/index.ts")));
    assert!(globset.is_match(Path::new("deep/nested/file.ts")));
    assert!(!globset.is_match(Path::new("file.js")));
}

#[test]
fn test_multiple_extensions() {
    let globset = compile_globset(&["**/*.{ts,tsx}"]);

    assert!(globset.is_match(Path::new("src/index.ts")));
    assert!(globset.is_match(Path::new("src/component.tsx")));
    assert!(!globset.is_match(Path::new("src/file.js")));
}

#[test]
fn test_exclude_patterns() {
    let include = compile_globset(&["**/*.ts"]);
    let exclude = compile_globset(&["**/node_modules/**", "**/dist/**"]);

    assert!(include.is_match(Path::new("node_modules/pkg/index.ts")));
    assert!(exclude.is_match(Path::new("node_modules/pkg/index.ts")));

    assert!(include.is_match(Path::new("dist/bundle.ts")));
    assert!(exclude.is_match(Path::new("dist/bundle.ts")));

    assert!(include.is_match(Path::new("src/index.ts")));
    assert!(!exclude.is_match(Path::new("src/index.ts")));
}

#[test]
fn test_rule_specific_patterns_intersection() {
    let global_include = compile_globset(&["**/*.ts", "**/*.tsx"]);
    let global_exclude = compile_globset(&["**/node_modules/**"]);
    let rule_include = compile_globset(&["src/**"]);

    let test_cases = vec![
        ("src/file.ts", true),
        ("src/nested/file.ts", true),
        ("lib/file.ts", false),
        ("test/file.ts", false),
        ("src/node_modules/pkg.ts", false),
    ];

    for (path, should_match) in test_cases {
        let path = Path::new(path);
        let matches = global_include.is_match(path)
            && !global_exclude.is_match(path)
            && rule_include.is_match(path);

        assert_eq!(
            matches,
            should_match,
            "Path {} should {}match",
            path.display(),
            if should_match { "" } else { "not " }
        );
    }
}

#[test]
fn test_rule_exclude_patterns() {
    let global_include = compile_globset(&["**/*.ts"]);
    let global_exclude = compile_globset(&["**/node_modules/**"]);
    let rule_exclude = compile_globset(&["**/*.test.ts"]);

    let test_cases = vec![
        ("src/index.ts", true),
        ("src/index.test.ts", false),
        ("test/example.test.ts", false),
        ("test/utils.ts", true),
        ("node_modules/pkg/file.ts", false),
    ];

    for (path, should_match) in test_cases {
        let path = Path::new(path);
        let matches = global_include.is_match(path)
            && !global_exclude.is_match(path)
            && !rule_exclude.is_match(path);

        assert_eq!(
            matches,
            should_match,
            "Path {} should {}match",
            path.display(),
            if should_match { "" } else { "not " }
        );
    }
}

#[test]
fn test_empty_rule_patterns_use_global() {
    let global_include = compile_globset(&["**/*.ts", "**/*.tsx"]);
    let global_exclude = compile_globset(&["**/node_modules/**"]);

    let test_cases = vec![
        ("src/index.ts", true),
        ("src/component.tsx", true),
        ("node_modules/pkg/file.ts", false),
        ("lib/utils.js", false),
    ];

    for (path, should_match) in test_cases {
        let path = Path::new(path);
        let matches = global_include.is_match(path) && !global_exclude.is_match(path);

        assert_eq!(
            matches,
            should_match,
            "Path {} should {}match",
            path.display(),
            if should_match { "" } else { "not " }
        );
    }
}

#[test]
fn test_pattern_with_leading_dot_slash() {
    let globset_without_dot = compile_globset(&["src/**"]);
    let globset_with_dot = compile_globset(&["./src/**"]);

    assert!(globset_without_dot.is_match(Path::new("src/file.ts")));
    assert!(globset_without_dot.is_match(Path::new("src/nested/file.ts")));

    assert!(!globset_with_dot.is_match(Path::new("src/file.ts")));
    assert!(globset_with_dot.is_match(Path::new("./src/file.ts")));
}

#[test]
fn test_nested_directory_exclusion() {
    let exclude = compile_globset(&["**/dist/**", "**/.git/**"]);

    assert!(exclude.is_match(Path::new("dist/bundle.ts")));
    assert!(exclude.is_match(Path::new("packages/core/dist/index.ts")));
    assert!(exclude.is_match(Path::new(".git/hooks/pre-commit")));
    assert!(!exclude.is_match(Path::new("src/dist-utils.ts")));
}

#[test]
fn test_specific_directory_only() {
    let include = compile_globset(&["src/**/*.ts"]);

    assert!(include.is_match(Path::new("src/index.ts")));
    assert!(include.is_match(Path::new("src/deep/nested/file.ts")));
    assert!(!include.is_match(Path::new("lib/index.ts")));
    assert!(!include.is_match(Path::new("test/example.ts")));
}

#[test]
fn test_root_level_files() {
    let include = compile_globset(&["*.ts"]);

    assert!(include.is_match(Path::new("index.ts")));
    assert!(include.is_match(Path::new("src/index.ts")));

    let root_only = compile_globset(&["[!.]*.ts"]);
    assert!(root_only.is_match(Path::new("index.ts")));
    assert!(!root_only.is_match(Path::new("./index.ts")));
}

#[test]
fn test_combined_rule_and_global_intersection() {
    let global_include = compile_globset(&["**/*.ts", "**/*.tsx"]);
    let global_exclude = compile_globset(&["**/node_modules/**", "**/dist/**"]);
    let rule_include = compile_globset(&["src/**", "lib/**"]);
    let rule_exclude = compile_globset(&["**/*.test.ts", "**/*.spec.ts"]);

    let test_cases = vec![
        ("src/index.ts", true),
        ("lib/utils.ts", true),
        ("src/component.tsx", true),
        ("src/index.test.ts", false),
        ("lib/utils.spec.ts", false),
        ("test/example.ts", false),
        ("node_modules/pkg/file.ts", false),
        ("dist/bundle.ts", false),
        ("src/dist/output.ts", false),
    ];

    for (path, should_match) in test_cases {
        let path = Path::new(path);
        let matches = global_include.is_match(path)
            && !global_exclude.is_match(path)
            && rule_include.is_match(path)
            && !rule_exclude.is_match(path);

        assert_eq!(
            matches,
            should_match,
            "Path {} should {}match with combined rules",
            path.display(),
            if should_match { "" } else { "not " }
        );
    }
}

#[test]
fn test_case_sensitivity() {
    let globset = compile_globset(&["**/*.ts"]);

    assert!(globset.is_match(Path::new("file.ts")));
    assert!(!globset.is_match(Path::new("file.TS")));
    assert!(!globset.is_match(Path::new("file.Ts")));
}

#[test]
fn test_multiple_exclude_patterns() {
    let exclude = compile_globset(&[
        "**/node_modules/**",
        "**/dist/**",
        "**/build/**",
        "**/.git/**",
    ]);

    let excluded_paths = vec![
        "node_modules/pkg/index.ts",
        "dist/bundle.ts",
        "build/output.ts",
        ".git/config",
        "packages/core/node_modules/dep/file.ts",
    ];

    for path in excluded_paths {
        assert!(
            exclude.is_match(Path::new(path)),
            "Path {} should be excluded",
            path
        );
    }

    let included_paths = vec!["src/index.ts", "lib/utils.ts", "test/example.ts"];

    for path in included_paths {
        assert!(
            !exclude.is_match(Path::new(path)),
            "Path {} should not be excluded",
            path
        );
    }
}
