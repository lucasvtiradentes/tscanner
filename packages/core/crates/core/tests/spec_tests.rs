use core::{parse_file, FileSource, Issue, RuleRegistration};
use std::fs;
use std::path::Path;

fn run_rule_test(input_path: &str) {
    let input_file = Path::new(input_path);
    let file_name = input_file.file_name().unwrap().to_str().unwrap();

    let rule_name = input_file
        .parent()
        .and_then(|p| p.file_name())
        .and_then(|n| n.to_str())
        .expect("test file must be in a rule directory (specs/{rule-name}/)");

    let source = fs::read_to_string(input_file)
        .unwrap_or_else(|err| panic!("failed to read {input_file:?}: {err:?}"));

    let program = parse_file(input_file, &source)
        .unwrap_or_else(|err| panic!("failed to parse {input_file:?}: {err:?}"));

    let file_source = FileSource::from_path(input_file);

    let rule = inventory::iter::<RuleRegistration>
        .into_iter()
        .find(|r| r.name == rule_name)
        .map(|r| (r.factory)(None))
        .unwrap_or_else(|| panic!("rule '{rule_name}' not found"));

    let mut issues = rule.check(&program, input_file, &source, file_source);
    issues.sort_by(|a, b| a.line.cmp(&b.line).then(a.column.cmp(&b.column)));

    let snapshot = format_snapshot(&source, &issues, input_file);

    insta::with_settings!({
        prepend_module_to_snapshot => false,
        snapshot_path => input_file.parent().unwrap(),
        snapshot_suffix => "",
    }, {
        insta::assert_snapshot!(file_name, snapshot);
    });
}

fn format_snapshot(source: &str, issues: &[Issue], path: &Path) -> String {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("ts");
    let mut output = String::new();

    output.push_str(&format!("# Input\n```{}\n", ext));
    output.push_str(source);
    if !source.ends_with('\n') {
        output.push('\n');
    }
    output.push_str("```\n\n");

    output.push_str("# Diagnostics\n");

    if issues.is_empty() {
        output.push_str("No issues found.\n");
    } else {
        for issue in issues {
            output.push_str(&format!(
                "```\n{}:{}:{} {} ━━━━━━━━━━━━━━━━━━━━\n\n",
                issue.file.file_name().unwrap_or_default().to_string_lossy(),
                issue.line,
                issue.column,
                issue.rule
            ));
            output.push_str(&format!("  ! {}\n\n", issue.message));

            let lines: Vec<&str> = source.lines().collect();
            if issue.line > 0 && issue.line <= lines.len() {
                let line_idx = issue.line - 1;

                if line_idx > 0 {
                    output.push_str(&format!(
                        "    {} │ {}\n",
                        issue.line - 1,
                        lines.get(line_idx - 1).unwrap_or(&"")
                    ));
                }

                output.push_str(&format!(
                    "  > {} │ {}\n",
                    issue.line,
                    lines.get(line_idx).unwrap_or(&"")
                ));

                if line_idx + 1 < lines.len() {
                    output.push_str(&format!(
                        "    {} │ {}\n",
                        issue.line + 1,
                        lines.get(line_idx + 1).unwrap_or(&"")
                    ));
                }
            }

            output.push_str("\n```\n\n");
        }
    }

    output
}

macro_rules! generate_rule_tests {
    ($rule_name:ident) => {
        mod $rule_name {
            use super::run_rule_test;

            #[test]
            fn invalid_ts() {
                let rule_dir = stringify!($rule_name).replace('_', "-");
                let path = format!(
                    "{}/tests/specs/{}/invalid.ts",
                    env!("CARGO_MANIFEST_DIR"),
                    rule_dir
                );
                if std::path::Path::new(&path).exists() {
                    run_rule_test(&path);
                }
            }

            #[test]
            fn valid_ts() {
                let rule_dir = stringify!($rule_name).replace('_', "-");
                let path = format!(
                    "{}/tests/specs/{}/valid.ts",
                    env!("CARGO_MANIFEST_DIR"),
                    rule_dir
                );
                if std::path::Path::new(&path).exists() {
                    run_rule_test(&path);
                }
            }

            #[test]
            fn invalid_js() {
                let rule_dir = stringify!($rule_name).replace('_', "-");
                let path = format!(
                    "{}/tests/specs/{}/invalid.js",
                    env!("CARGO_MANIFEST_DIR"),
                    rule_dir
                );
                if std::path::Path::new(&path).exists() {
                    run_rule_test(&path);
                }
            }

            #[test]
            fn valid_js() {
                let rule_dir = stringify!($rule_name).replace('_', "-");
                let path = format!(
                    "{}/tests/specs/{}/valid.js",
                    env!("CARGO_MANIFEST_DIR"),
                    rule_dir
                );
                if std::path::Path::new(&path).exists() {
                    run_rule_test(&path);
                }
            }
        }
    };
}

generate_rule_tests!(no_any_type);
generate_rule_tests!(no_console_log);
generate_rule_tests!(no_var);
generate_rule_tests!(prefer_const);
generate_rule_tests!(no_empty_function);
generate_rule_tests!(no_empty_class);
generate_rule_tests!(no_nested_ternary);
generate_rule_tests!(no_unreachable_code);
generate_rule_tests!(no_constant_condition);
generate_rule_tests!(consistent_return);
generate_rule_tests!(no_default_export);
generate_rule_tests!(no_duplicate_imports);
generate_rule_tests!(no_dynamic_import);
generate_rule_tests!(no_else_return);
generate_rule_tests!(no_relative_imports);
generate_rule_tests!(no_absolute_imports);
generate_rule_tests!(no_alias_imports);
generate_rule_tests!(no_async_without_await);
generate_rule_tests!(no_shadow);
generate_rule_tests!(no_single_or_array_union);
generate_rule_tests!(no_nested_require);
generate_rule_tests!(no_todo_comments);
generate_rule_tests!(no_unused_vars);
generate_rule_tests!(no_implicit_any);
generate_rule_tests!(no_forwarded_exports);
generate_rule_tests!(no_unnecessary_type_assertion);
generate_rule_tests!(prefer_type_over_interface);
generate_rule_tests!(max_params);
generate_rule_tests!(prefer_interface_over_type);
generate_rule_tests!(max_function_length);
generate_rule_tests!(prefer_optional_chain);
generate_rule_tests!(prefer_nullish_coalescing);
generate_rule_tests!(no_floating_promises);
generate_rule_tests!(no_useless_catch);
generate_rule_tests!(no_return_await);
generate_rule_tests!(no_empty_interface);
generate_rule_tests!(no_inferrable_types);
generate_rule_tests!(no_non_null_assertion);
