---
'tscanner': minor
---

Remove the built-in `prefer-nullish-coalescing` rule. The AST-only heuristic could not distinguish nullish-default `||` from boolean OR, empty-string default patterns (`@actions/core.getInput() || default`), or env-var truthiness checks, producing too many false positives to be useful. Users referencing this rule in their `.tscanner/config.jsonc` should remove the entry.
