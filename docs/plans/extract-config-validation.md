# Plan: Extract Config Validation Logic

## Problem

Config validation currently runs automatically inside `TscannerConfig::load_from_file()`. The new `tscanner config --validate` command just loads the config (triggering validation internally) and prints success. This is redundant and doesn't allow fine-grained control over validation output.

## Current State

### Validation Flow (config.rs)

```rust
// config.rs:490-500
pub fn load_from_file(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let json_without_comments = json_comments::StripComments::new(content.as_bytes());
    let json_value: serde_json::Value = serde_json::from_reader(json_without_comments)?;

    validate_json_fields(&json_value)?;  // Step 1: JSON schema validation

    let config: Self = serde_json::from_value(json_value)?;
    config.validate()?;  // Step 2: Semantic validation
    Ok(config)
}
```

### Two Validation Functions

1. **`validate_json_fields()`** (config.rs:92-137)
   - Validates JSON structure against allowed fields
   - Checks top-level, cli, files, codeEditor, builtinRules, customRules
   - Returns formatted error with invalid field names

2. **`TscannerConfig::validate()`** (config.rs:503-575)
   - Semantic validation (regex patterns, required fields per rule type)
   - Checks conflicting rules (e.g., prefer-type-over-interface vs prefer-interface-over-type)
   - Prints warnings to stderr

### Current cmd_validate (commands/config/command.rs:69-75)

```rust
fn cmd_validate(config_file_path: &str) -> Result<()> {
    println!("{} {}", "✓".green().bold(), "Configuration is valid".green());
    println!("  {}", config_file_path.dimmed());
    Ok(())
}
```

This does nothing - validation already happened in `load_config_with_custom()`.

## Goal

1. Make validation explicit and reusable
2. `--validate` should show detailed validation results
3. Avoid duplicate validation when just loading config

## Proposed Solution

### Option A: Lazy Validation (Recommended)

1. Add `load_from_file_without_validation()` that skips validation
2. Add public `validate()` method that runs both validations
3. `--validate` calls validate explicitly and formats output nicely
4. Other commands continue using `load_from_file()` (validates implicitly)

### Option B: Validation Result Type

1. Create `ValidationResult` struct with errors/warnings
2. Make validation return this instead of Result<(), Error>
3. Let callers decide how to handle/display

## Files to Change

| File | Changes |
|------|---------|
| `packages/core/crates/core/src/config.rs` | Add `load_without_validation()`, make `validate_json_fields()` public, refactor `validate()` to return structured result |
| `packages/core/crates/tscanner_cli/src/commands/config/command.rs` | Update `cmd_validate()` to run validation explicitly and show detailed output |
| `packages/core/crates/tscanner_cli/src/config_loader.rs` | Maybe add variant that skips validation |

## Implementation Steps

1. In `config.rs`:
   - Add `pub fn load_from_file_unchecked()` that doesn't validate
   - Keep `load_from_file()` as-is for backwards compat
   - Make `validate_json_fields()` public: `pub fn validate_json_fields()`
   - Add `pub fn full_validate(&self, json: &serde_json::Value) -> ValidationResult`

2. Create `ValidationResult` type:
   ```rust
   pub struct ValidationResult {
       pub errors: Vec<String>,
       pub warnings: Vec<String>,
   }

   impl ValidationResult {
       pub fn is_valid(&self) -> bool { self.errors.is_empty() }
   }
   ```

3. Update `cmd_validate()`:
   ```rust
   fn cmd_validate(config_file_path: &str) -> Result<()> {
       let content = fs::read_to_string(config_file_path)?;
       let json = parse_jsonc(&content)?;

       let result = validate_json_fields(&json);
       if !result.errors.is_empty() {
           // Print errors in red
           for err in &result.errors {
               eprintln!("{} {}", "✗".red(), err);
           }
           return Err(...);
       }

       let config: TscannerConfig = serde_json::from_value(json)?;
       let semantic_result = config.validate();

       for warn in &semantic_result.warnings {
           eprintln!("{} {}", "⚠".yellow(), warn);
       }

       if semantic_result.is_valid() {
           println!("{} Configuration is valid", "✓".green());
       }

       Ok(())
   }
   ```

## Expected Output

```bash
$ tscanner config --validate

# Success case:
✓ Configuration is valid
  Config: .tscanner/config.jsonc

# Warning case:
⚠ Conflicting rules enabled: 'prefer-type-over-interface' and 'prefer-interface-over-type'
✓ Configuration is valid (with warnings)
  Config: .tscanner/config.jsonc

# Error case:
✗ Invalid field: builtinRules.no-console.invalidOption
✗ Custom rule 'my-rule' is type 'regex' but has no 'pattern' field
  Config: .tscanner/config.jsonc
```

## Testing

1. Valid config → shows success
2. Invalid JSON field → shows field error
3. Invalid regex pattern → shows pattern error
4. Conflicting rules → shows warning but still valid
5. Missing required field → shows error

## Notes

- Keep backwards compatibility: `load_from_file()` still validates
- Consider adding `--verbose` flag for extra detail
- Server (LSP) uses same validation - might need similar changes there
