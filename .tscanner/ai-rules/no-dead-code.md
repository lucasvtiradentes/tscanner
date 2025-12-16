# Dead Code Detector

Detect dead code patterns in Rust codebases.

## What is Dead Code?

- **Unused variables** - Variables declared but never read
- **Unused functions** - Functions defined but never called
- **Unused imports** - `use` statements that are never referenced
- **Unused struct fields** - Fields that are never accessed (except via derive macros)
- **Unreachable code** - Code after `return`, `break`, `continue`, or `panic!`
- **Suppressed warnings** - Code with `#[allow(dead_code)]` or `#[allow(unused)]` annotations

## Rules

1. No `#[allow(dead_code)]` attributes - remove the dead code instead
2. No `#[allow(unused)]` or `#[allow(unused_variables)]` - use the variable or prefix with `_`
3. No variables prefixed with `_` that could be removed - just call the function if result isn't needed
4. No unreachable code after `return`, `break`, `continue`

## Instructions

1. Scan for `#[allow(dead_code)]` and similar attributes
2. Look for underscore-prefixed variables that suppress unused warnings
3. Check for unreachable code patterns
4. Report each violation with the exact line number

Only report actual violations. Focus on suppressions that hide real dead code issues.

---

## Options

{{OPTIONS}}

---

## Files

{{FILES}}
