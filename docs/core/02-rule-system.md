# Rule System

How to create and register rules in TScanner.

## Rule Trait

All rules implement the `Rule` trait:

```rust
pub trait Rule: Send + Sync {
    fn name(&self) → &str;
    fn check(&self, program: &Program, path: &Path, source: &str) → Vec<Issue>;
}
```

**Parameters:**
- `program` - SWC AST representation of the file
- `path` - File path being checked
- `source` - Original source code text

**Returns:**
- Vec of issues found in the file

## Rule Registration

Rules use the `inventory` crate for compile-time registration. No manual registration needed.

```rust
use crate::registry::RuleRegistration;

inventory::submit!(RuleRegistration {
    name: "my-rule",
    factory: || Arc::new(MyRule),
});
```

## Metadata Registration

Each rule also registers metadata for UI display:

```rust
use crate::registry::RuleMetadataRegistration;

inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "my-rule",
        display_name: "My Rule",
        description: "Description of what this rule checks",
        rule_type: RuleType::Ast,
        default_severity: Severity::Warning,
        default_enabled: false,
        category: RuleCategory::TypeSafety,
    }
});
```

**Categories:**
- `TypeSafety` - Type system issues
- `CodeQuality` - Code maintainability
- `Style` - Code formatting/conventions
- `Performance` - Performance issues
- `BugPrevention` - Potential bugs
- `Variables` - Variable declarations
- `Imports` - Import statements

## AST Rule Pattern

AST rules use SWC's visitor pattern to traverse the syntax tree.

```rust
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

pub struct MyRule;

impl Rule for MyRule {
    fn name(&self) → &str {
        "my-rule"
    }

    fn check(&self, program: &Program, path: &Path, source: &str) → Vec<Issue> {
        let mut visitor = MyRuleVisitor {
            issues: Vec::new(),
            source: source.to_string(),
        };
        program.visit_with(&mut visitor);
        visitor.issues
    }
}

struct MyRuleVisitor {
    issues: Vec<Issue>,
    source: String,
}

impl Visit for MyRuleVisitor {
    fn visit_ts_type_ann(&mut self, node: &TsTypeAnn) {
        if let TsType::TsKeywordType(keyword) = &*node.type_ann {
            if matches!(keyword.kind, TsKeywordTypeKind::TsAnyKeyword) {
                let (line, col) = get_line_col(&self.source, node.span.lo.0 as usize);
                self.issues.push(Issue {
                    rule: "my-rule".to_string(),
                    message: "Found any type".to_string(),
                    severity: Severity::Error,
                    line,
                    column: col,
                });
            }
        }
        node.visit_children_with(self);
    }
}
```

**Key methods:**
- `visit_ts_type_ann` - Type annotations
- `visit_var_decl` - Variable declarations
- `visit_fn_decl` - Function declarations
- `visit_expr` - Expressions
- `visit_stmt` - Statements
- `visit_import_decl` - Import statements

**Helper functions:**
- `get_line_col(source, byte_offset)` - Convert byte offset to line/column

## Regex Rule

Regex rules match patterns on source text. They are defined in config, not code:

```json
{
  "rules": {
    "custom-pattern": {
      "enabled": true,
      "type": "regex",
      "severity": "warning",
      "pattern": "TODO:",
      "message": "Found TODO comment",
      "include": [],
      "exclude": []
    }
  }
}
```

The `RegexRule` struct handles pattern matching:

```rust
pub struct RegexRule {
    name: String,
    pattern: Regex,
    message: String,
    severity: Severity,
}
```

## File Structure

```
crates/core/src/rules/
├── mod.rs              # Module declarations
├── no_any_type.rs      # Example AST rule
├── no_console_log.rs   # Example AST rule
├── prefer_const.rs     # Example AST rule
└── ...
```

Add new rule to `mod.rs`:

```rust
mod my_rule;
pub use my_rule::MyRule;
```

## Testing

See [Testing Guide](06-testing.md) for how to test rules.
