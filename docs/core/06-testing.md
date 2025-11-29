# Testing Rules with Snapshot Tests

Testing framework: [insta](https://insta.rs/) (snapshot testing)

## Test File Structure

```
crates/core/tests/
├── spec_tests.rs              # Rule snapshot test runner
├── glob_tests.rs              # Glob pattern matching unit tests
├── integration_glob_tests.rs  # Config glob integration tests
└── specs/
    └── {rule-name}/
        ├── invalid.ts     # Code that should trigger the rule
        ├── invalid.ts.snap # Expected snapshot
        ├── valid.ts       # Code that should NOT trigger
        └── valid.ts.snap  # Expected snapshot (no issues)
```

## How It Works

The `spec_tests.rs` file:
- Macro generates tests for each rule
- Reads `.ts` file, parses with SWC
- Finds rule by directory name
- Runs `rule.check()`
- Compares output with `.snap` file

## Snapshot Format

```markdown
# Input
\`\`\`ts
{source code}
\`\`\`

# Diagnostics
\`\`\`
{file}:{line}:{col} {rule} ━━━━━━━━━━━━━━━━━━━━

  ! {message}

    {line-1} │ {code}
  > {line}   │ {highlighted code}
    {line+1} │ {code}
\`\`\`
```

## Adding Tests for a New Rule

```bash
# 1. Create test directory
mkdir crates/core/tests/specs/my-new-rule

# 2. Create test files
echo "code that triggers rule" > crates/core/tests/specs/my-new-rule/invalid.ts
echo "code that passes" > crates/core/tests/specs/my-new-rule/valid.ts

# 3. Add to spec_tests.rs
generate_rule_tests!(my_new_rule);  # Note: underscores, not hyphens

# 4. Run tests to generate snapshots
cargo test -p core

# 5. Review and accept snapshots
cargo insta review
```

## Running Tests

```bash
# Run all tests
cargo test -p core

# Run specific rule test
cargo test -p core no_any_type

# Update snapshots
cargo insta review
```

## Test Macro

```rust
generate_rule_tests!(rule_name);
```

This macro generates tests for both `invalid.ts` and `valid.ts` in the rule's spec directory.

## Glob Pattern Tests

### Unit Tests (glob_tests.rs)

Tests for raw `globset` pattern matching:

```rust
#[test]
fn test_basic_pattern_matching() {
    let globset = compile_globset(&["**/*.ts"]);
    assert!(globset.is_match(Path::new("src/index.ts")));
    assert!(!globset.is_match(Path::new("file.js")));
}

#[test]
fn test_rule_specific_patterns_intersection() {
    let global_include = compile_globset(&["**/*.ts"]);
    let rule_include = compile_globset(&["src/**"]);

    // Must match BOTH global AND rule
    let path = Path::new("src/file.ts");
    let matches = global_include.is_match(path) && rule_include.is_match(path);
    assert!(matches);
}
```

### Integration Tests (integration_glob_tests.rs)

Tests using actual `TscannerConfig`:

```rust
#[test]
fn test_rule_specific_include_intersects_with_global() {
    let config = TscannerConfig {
        include: vec!["**/*.ts".to_string()],
        exclude: vec!["**/node_modules/**".to_string()],
        builtin_rules: [(
            "no-any-type".to_string(),
            BuiltinRuleConfig {
                enabled: Some(true),
                include: vec!["src/**".to_string()],
                exclude: vec![],
                ..Default::default()
            },
        )].into_iter().collect(),
        ..Default::default()
    };

    let compiled = config.compile_builtin_rule("no-any-type").unwrap();

    assert!(compiled.matches(Path::new("src/index.ts")));
    assert!(!compiled.matches(Path::new("lib/utils.ts"))); // not in src/
}
```

## Running Tests

```bash
# Run all tests
cargo test -p core

# Run only spec tests
cargo test -p core --test spec_tests

# Run only glob tests
cargo test -p core --test glob_tests

# Run only integration glob tests
cargo test -p core --test integration_glob_tests
```

## Related Documentation

- [Rule System](02-rule-system.md)
- [Scanner Flow](03-scanner-flow.md) - Glob pattern matching section
