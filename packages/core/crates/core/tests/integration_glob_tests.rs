use core::config::{BuiltinRuleConfig, FilesConfig, TscannerConfig};
use std::path::Path;

#[test]
fn test_global_patterns_only() {
    let config = TscannerConfig {
        schema: None,
        lsp: None,
        cli: None,
        builtin_rules: [(
            "no-any-type".to_string(),
            BuiltinRuleConfig {
                enabled: Some(true),
                severity: None,
                include: vec![],
                exclude: vec![],
            },
        )]
        .into_iter()
        .collect(),
        custom_rules: Default::default(),
        files: FilesConfig {
            include: vec!["**/*.ts".to_string(), "**/*.tsx".to_string()],
            exclude: vec!["**/node_modules/**".to_string(), "**/dist/**".to_string()],
        },
    };

    let compiled = config
        .compile_builtin_rule("no-any-type")
        .expect("Failed to compile rule");

    let test_cases = vec![
        ("src/index.ts", true),
        ("src/component.tsx", true),
        ("node_modules/pkg/file.ts", false),
        ("dist/bundle.ts", false),
        ("lib/utils.js", false),
    ];

    for (path, should_match) in test_cases {
        let path = Path::new(path);
        let matches = compiled.matches(path);
        assert_eq!(
            matches,
            should_match,
            "Path {} should {}match with global patterns only",
            path.display(),
            if should_match { "" } else { "not " }
        );
    }
}

#[test]
fn test_rule_specific_include_intersects_with_global() {
    let config = TscannerConfig {
        schema: None,
        lsp: None,
        cli: None,
        builtin_rules: [(
            "no-any-type".to_string(),
            BuiltinRuleConfig {
                enabled: Some(true),
                severity: None,
                include: vec!["src/**".to_string()],
                exclude: vec![],
            },
        )]
        .into_iter()
        .collect(),
        custom_rules: Default::default(),
        files: FilesConfig {
            include: vec!["**/*.ts".to_string(), "**/*.tsx".to_string()],
            exclude: vec!["**/node_modules/**".to_string()],
        },
    };

    let compiled = config
        .compile_builtin_rule("no-any-type")
        .expect("Failed to compile rule");

    let test_cases = vec![
        ("src/index.ts", true),
        ("src/nested/file.tsx", true),
        ("lib/utils.ts", false),
        ("test/example.ts", false),
        ("src/node_modules/pkg.ts", false),
    ];

    for (path, should_match) in test_cases {
        let path = Path::new(path);
        let matches = compiled.matches(path);
        assert_eq!(
            matches,
            should_match,
            "Path {} should {}match with rule-specific include",
            path.display(),
            if should_match { "" } else { "not " }
        );
    }
}

#[test]
fn test_rule_specific_exclude_adds_to_global() {
    let config = TscannerConfig {
        schema: None,
        lsp: None,
        cli: None,
        builtin_rules: [(
            "no-any-type".to_string(),
            BuiltinRuleConfig {
                enabled: Some(true),
                severity: None,
                include: vec![],
                exclude: vec!["**/*.test.ts".to_string()],
            },
        )]
        .into_iter()
        .collect(),
        custom_rules: Default::default(),
        files: FilesConfig {
            include: vec!["**/*.ts".to_string()],
            exclude: vec!["**/node_modules/**".to_string()],
        },
    };

    let compiled = config
        .compile_builtin_rule("no-any-type")
        .expect("Failed to compile rule");

    let test_cases = vec![
        ("src/index.ts", true),
        ("src/index.test.ts", false),
        ("test/example.test.ts", false),
        ("test/utils.ts", true),
        ("node_modules/pkg/file.ts", false),
    ];

    for (path, should_match) in test_cases {
        let path = Path::new(path);
        let matches = compiled.matches(path);
        assert_eq!(
            matches,
            should_match,
            "Path {} should {}match with rule-specific exclude",
            path.display(),
            if should_match { "" } else { "not " }
        );
    }
}

#[test]
fn test_combined_rule_include_and_exclude() {
    let config = TscannerConfig {
        schema: None,
        lsp: None,
        cli: None,
        builtin_rules: [(
            "no-any-type".to_string(),
            BuiltinRuleConfig {
                enabled: Some(true),
                severity: None,
                include: vec!["src/**".to_string(), "lib/**".to_string()],
                exclude: vec!["**/*.test.ts".to_string(), "**/*.spec.ts".to_string()],
            },
        )]
        .into_iter()
        .collect(),
        custom_rules: Default::default(),
        files: FilesConfig {
            include: vec!["**/*.ts".to_string()],
            exclude: vec!["**/node_modules/**".to_string(), "**/dist/**".to_string()],
        },
    };

    let compiled = config
        .compile_builtin_rule("no-any-type")
        .expect("Failed to compile rule");

    let test_cases = vec![
        ("src/index.ts", true),
        ("lib/utils.ts", true),
        ("src/component.test.ts", false),
        ("lib/helper.spec.ts", false),
        ("test/example.ts", false),
        ("node_modules/pkg/file.ts", false),
        ("dist/bundle.ts", false),
    ];

    for (path, should_match) in test_cases {
        let path = Path::new(path);
        let matches = compiled.matches(path);
        assert_eq!(
            matches,
            should_match,
            "Path {} should {}match with combined patterns",
            path.display(),
            if should_match { "" } else { "not " }
        );
    }
}

#[test]
fn test_path_normalization_with_root() {
    let config = TscannerConfig {
        schema: None,
        lsp: None,
        cli: None,
        builtin_rules: [(
            "no-any-type".to_string(),
            BuiltinRuleConfig {
                enabled: Some(true),
                severity: None,
                include: vec!["src/**".to_string()],
                exclude: vec![],
            },
        )]
        .into_iter()
        .collect(),
        custom_rules: Default::default(),
        files: FilesConfig {
            include: vec!["**/*.ts".to_string()],
            exclude: vec![],
        },
    };

    let compiled = config
        .compile_builtin_rule("no-any-type")
        .expect("Failed to compile rule");

    let root = Path::new("/home/user/project");
    let absolute_path = root.join("src/file.ts");

    let relative = absolute_path.strip_prefix(root).unwrap_or(&absolute_path);

    assert!(compiled.matches(relative));
}
