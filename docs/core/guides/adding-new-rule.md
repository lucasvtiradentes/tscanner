# Adding a New Rule

Step-by-step guide for creating and testing a new rule in TScanner.

## Prerequisites

**Required knowledge:**
- Basic Rust syntax (traits, structs, pattern matching)
- SWC AST structure (explore at [astexplorer.net](https://astexplorer.net/#/gist/0a92bbf654aca4fdfb3730ef1cf5ebb7/5efc259cd16c7f475a5dc1ec04ceb15f11f06b35))
- TypeScript/JavaScript syntax (what your rule checks for)

**Helpful commands:**
```bash
cd packages/core/crates/core
cargo test -p core        # Run all tests
cargo insta review        # Review/accept snapshots
```

## Step 1: Create the Rule File

**Location:** `crates/core/src/rules/{rule_name}.rs`

**Naming:** Use snake_case (e.g., `no_debugger.rs`, `prefer_const.rs`)

**Template:**

```rust
use crate::rules::metadata::RuleType;
use crate::rules::{Rule, RuleCategory, RuleMetadata, RuleMetadataRegistration, RuleRegistration};
use crate::types::{Issue, Severity};
use crate::utils::get_line_col;
use std::path::Path;
use std::sync::Arc;
use swc_common::Spanned;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

pub struct NoDebuggerRule;

impl Rule for NoDebuggerRule {
    fn name(&self) -> &str {
        "no-debugger"
    }

    fn check(&self, program: &Program, path: &Path, source: &str) -> Vec<Issue> {
        let mut visitor = NoDebuggerVisitor {
            issues: Vec::new(),
            path: path.to_path_buf(),
            source,
        };
        program.visit_with(&mut visitor);
        visitor.issues
    }
}

struct NoDebuggerVisitor<'a> {
    issues: Vec<Issue>,
    path: std::path::PathBuf,
    source: &'a str,
}

impl<'a> Visit for NoDebuggerVisitor<'a> {
    fn visit_debugger_stmt(&mut self, n: &DebuggerStmt) {
        let span = n.span();
        let (line, column) = get_line_col(self.source, span.lo.0 as usize);

        self.issues.push(Issue {
            rule: "no-debugger".to_string(),
            file: self.path.clone(),
            line,
            column,
            message: "Debugger statement should not be used in production".to_string(),
            severity: Severity::Error,
            line_text: None,
        });

        n.visit_children_with(self);
    }
}
```

## Step 2: Register the Rule

**Add two inventory submissions:**

```rust
inventory::submit!(RuleRegistration {
    name: "no-debugger",
    factory: || Arc::new(NoDebuggerRule),
});

inventory::submit!(RuleMetadataRegistration {
    metadata: RuleMetadata {
        name: "no-debugger",
        display_name: "No Debugger",
        description: "Disallows debugger statements in production code",
        rule_type: RuleType::Ast,
        default_severity: Severity::Error,
        default_enabled: false,
        category: RuleCategory::BugPrevention,
    }
});
```

**Categories:** `TypeSafety`, `CodeQuality`, `Style`, `Performance`, `BugPrevention`, `Variables`, `Imports`

**Add to `rules/mod.rs`:**

```rust
mod no_debugger;
```

## Step 3: Implement Common Visit Methods

**Common visitor methods:**

```rust
impl<'a> Visit for MyVisitor<'a> {
    fn visit_var_decl(&mut self, n: &VarDecl) {
        // Variable declarations (let, const, var)
        n.visit_children_with(self);
    }

    fn visit_fn_decl(&mut self, n: &FnDecl) {
        // Function declarations
        n.visit_children_with(self);
    }

    fn visit_call_expr(&mut self, n: &CallExpr) {
        // Function calls (e.g., console.log())
        n.visit_children_with(self);
    }

    fn visit_ts_type_ann(&mut self, n: &TsTypeAnn) {
        // Type annotations (: Type)
        n.visit_children_with(self);
    }

    fn visit_ts_keyword_type(&mut self, n: &TsKeywordType) {
        // Keyword types (any, unknown, never, etc.)
        n.visit_children_with(self);
    }

    fn visit_import_decl(&mut self, n: &ImportDecl) {
        // Import statements
        n.visit_children_with(self);
    }

    fn visit_assign_expr(&mut self, n: &AssignExpr) {
        // Assignments (x = y)
        n.visit_children_with(self);
    }
}
```

**Helper functions:**
- `get_line_col(source, byte_offset)` - Convert byte offset to line/column
- `n.span()` - Get span information from any node
- `n.visit_children_with(self)` - Continue visiting child nodes

## Step 4: Create Test Specs

**Create test directory:**

```bash
mkdir crates/core/tests/specs/no-debugger
```

**Create `invalid.ts` (code that should trigger):**

```typescript
function debug() {
  debugger;
}

debugger;
```

**Create `valid.ts` (code that should pass):**

```typescript
function debug() {
  console.log('debugging');
}

const debugger = 'variable name is ok';
```

**Add test macro to `spec_tests.rs`:**

```rust
generate_rule_tests!(no_debugger);
```

Note: Use underscores, not hyphens (converts automatically).

## Step 5: Run and Verify

**Generate snapshots:**

```bash
cd packages/core/crates/core
cargo test -p core
```

**Review snapshots:**

```bash
cargo insta review
```

Press:
- `a` - Accept snapshot
- `r` - Reject snapshot
- `s` - Skip

**Verify snapshot format:**

```markdown
# Input
\`\`\`ts
debugger;
\`\`\`

# Diagnostics
\`\`\`
invalid.ts:1:1 no-debugger ━━━━━━━━━━━━━━━━━━━━

  ! Debugger statement should not be used in production

  > 1 │ debugger;

\`\`\`
```

## Example: Two-Pass Analysis

Some rules need two passes (e.g., `prefer-const` checks declarations then reassignments):

```rust
impl Rule for PreferConstRule {
    fn check(&self, program: &Program, path: &Path, source: &str) -> Vec<Issue> {
        let mut collector = VariableCollector {
            let_declarations: HashMap::new(),
            source,
        };
        program.visit_with(&mut collector);

        let mut checker = ReassignmentChecker {
            reassigned: HashSet::new(),
        };
        program.visit_with(&mut checker);

        let mut issues = Vec::new();
        for (name, (line, column)) in collector.let_declarations {
            if !checker.reassigned.contains(&name) {
                issues.push(Issue {
                    rule: "prefer-const".to_string(),
                    file: path.to_path_buf(),
                    line,
                    column,
                    message: format!("'{}' is never reassigned, use 'const' instead", name),
                    severity: Severity::Warning,
                    line_text: None,
                });
            }
        }
        issues
    }
}
```

## Regex Rules (Alternative)

For simple pattern matching without AST:

```rust
use regex::Regex;

impl Rule for NoConsoleLogRule {
    fn check(&self, _program: &Program, path: &Path, source: &str) -> Vec<Issue> {
        let regex = Regex::new(r"console\.log\(").unwrap();
        let mut issues = Vec::new();

        for (line_num, line) in source.lines().enumerate() {
            if let Some(mat) = regex.find(line) {
                issues.push(Issue {
                    rule: self.name().to_string(),
                    file: path.to_path_buf(),
                    line: line_num + 1,
                    column: mat.start() + 1,
                    message: "Avoid using console.log in production code".to_string(),
                    severity: Severity::Warning,
                    line_text: None,
                });
            }
        }
        issues
    }
}
```

Set `rule_type: RuleType::Regex` in metadata.

## Checklist

- [ ] Create rule file in `crates/core/src/rules/`
- [ ] Implement `Rule` trait with `name()` and `check()`
- [ ] Add `inventory::submit!` for `RuleRegistration`
- [ ] Add `inventory::submit!` for `RuleMetadataRegistration`
- [ ] Add `mod` declaration to `rules/mod.rs`
- [ ] Create `specs/{rule-name}/` directory
- [ ] Create `invalid.ts` test file
- [ ] Create `valid.ts` test file
- [ ] Add `generate_rule_tests!` macro to `spec_tests.rs`
- [ ] Run `cargo test -p core`
- [ ] Review snapshots with `cargo insta review`
- [ ] Verify snapshots match expected output

## Related Documentation

- [Rule System](../02-rule-system.md) - Rule trait and registration
- [Testing](../06-testing.md) - Snapshot testing framework
- [SWC AST Explorer](https://astexplorer.net/#/gist/0a92bbf654aca4fdfb3730ef1cf5ebb7/5efc259cd16c7f475a5dc1ec04ceb15f11f06b35) - Explore TypeScript AST structure
