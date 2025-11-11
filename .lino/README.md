# Lino Configuration

This directory contains Lino's configuration files.

## rules.json

Configure which rules to enable/disable and customize their behavior.

### Structure

```json
{
  "rules": {
    "rule-name": {
      "enabled": true,
      "type": "ast" | "regex",
      "severity": "error" | "warning",
      "message": "Custom error message",
      "include": ["glob patterns"],
      "exclude": ["glob patterns"],
      "pattern": "regex pattern (for regex rules)"
    }
  },
  "include": ["global include patterns"],
  "exclude": ["global exclude patterns"]
}
```

### Available Rules

#### no-any-type (AST)
Detects TypeScript `any` type usage (`: any` and `as any`).
- **Type**: AST-based
- **Default severity**: Error
- **Example**: `const x: any = 1` → ❌

#### no-console-log (Regex)
Finds `console.log()` statements in code.
- **Type**: Regex-based
- **Default severity**: Warning
- **Example**: `console.log('debug')` → ⚠️

#### no-relative-imports (AST)
Detects relative imports (starting with `./` or `../`).
- **Type**: AST-based
- **Default severity**: Warning
- **Example**: `import { foo } from './bar'` → ⚠️

#### prefer-type-over-interface (AST)
Suggests using `type` instead of `interface` for consistency.
- **Type**: AST-based
- **Default severity**: Warning
- **Example**: `interface User { }` → ⚠️ (prefer `type User = { }`)

### Configuration Examples

#### Enable all rules
```json
{
  "rules": {
    "no-any-type": { "enabled": true, "type": "ast", "severity": "error" },
    "no-console-log": { "enabled": true, "type": "regex", "severity": "warning" },
    "no-relative-imports": { "enabled": true, "type": "ast", "severity": "warning" },
    "prefer-type-over-interface": { "enabled": true, "type": "ast", "severity": "warning" }
  }
}
```

#### Only check for `any` types in source files
```json
{
  "rules": {
    "no-any-type": {
      "enabled": true,
      "type": "ast",
      "severity": "error",
      "include": ["src/**/*.ts"],
      "exclude": ["src/**/*.test.ts"]
    }
  }
}
```

#### Custom severity levels
```json
{
  "rules": {
    "no-console-log": {
      "enabled": true,
      "type": "regex",
      "severity": "error",
      "message": "Console.log is forbidden in production code"
    }
  }
}
```

### Glob Patterns

- `**/*.ts` - All TypeScript files
- `src/**/*.tsx` - All TSX files in src directory
- `!**/*.test.ts` - Exclude test files
- `{src,lib}/**/*.ts` - Multiple directories

### Per-Rule vs Global Patterns

- **Per-rule patterns**: Override global patterns for specific rules
- **Global patterns**: Apply to all rules unless overridden

```json
{
  "include": ["**/*.ts"],
  "exclude": ["**/*.test.ts"],
  "rules": {
    "no-any-type": {
      "enabled": true,
      "type": "ast",
      "severity": "error",
      "include": ["src/**/*.ts"]
    }
  }
}
```

In this example:
- Global: scan all `.ts` files except tests
- `no-any-type` rule: only scan `src/**/*.ts` files

### Default Configuration

If no `.lino/rules.json` exists, Lino uses this default:

```json
{
  "rules": {
    "no-any-type": {
      "enabled": true,
      "type": "ast",
      "severity": "error",
      "message": "Found 'any' type annotation"
    }
  },
  "include": ["**/*.{ts,tsx}"],
  "exclude": ["node_modules/**", "dist/**", "build/**", ".git/**"]
}
```
