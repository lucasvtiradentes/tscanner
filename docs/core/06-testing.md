# Testing Rules with Snapshot Tests

Testing framework: [insta](https://insta.rs/) (snapshot testing)

## Test File Structure

```
crates/core/tests/
├── spec_tests.rs          # Test runner
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

## Related Documentation

- [Rule System](02-rule-system.md)
