# TypeScript Implicit Any Detector

Find TypeScript implicit-any diagnostics by delegating to the TypeScript compiler.

## Source of truth

Do not infer this rule from source code manually. Use TypeScript diagnostics as the source of truth.

Run this command from the repository root:

```bash
npx tsc --noEmit --noImplicitAny --pretty false --incremental false
```

If the repository has multiple TypeScript projects, run the equivalent command for the relevant `tsconfig.json` files that cover the scoped files.

## Diagnostics to report

Report only TypeScript diagnostics related to implicit-any parameters or binding elements:

- `TS7006`: parameter implicitly has an `any` type
- `TS7019`: rest parameter implicitly has an `any[]` type
- `TS7031`: binding element implicitly has an `any` type

Ignore every other TypeScript diagnostic code.

## Reporting rules

- Report only diagnostics whose file is in the provided scan scope.
- Use the exact file path, line, and column from the TypeScript diagnostic.
- The message should mention the TypeScript diagnostic code and tell the user to add an explicit type annotation.
- If TypeScript reports no matching diagnostics, return no issues.

## Instructions

1. Run the TypeScript command first.
2. Parse the compiler output.
3. Filter to `TS7006`, `TS7019`, and `TS7031`.
4. Convert matching diagnostics into the required JSON response format.
5. Do not report anything that does not come from TypeScript diagnostics.

---

## Files

{{FILES}}
