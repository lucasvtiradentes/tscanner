---
'tscanner': minor
---

Remove the built-in `no-implicit-any` rule. The AST-only heuristic could not match TypeScript `noImplicitAny` because contextual typing, generic inference, overloads, and declaration files require the TypeScript checker. Users referencing this rule in their `.tscanner/config.jsonc` should remove the entry.
