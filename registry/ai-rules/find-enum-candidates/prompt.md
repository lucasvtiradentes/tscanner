# Enum Candidates Detector

Find TypeScript type unions that could be replaced with enums for better type safety and maintainability.

## What to look for

1. String literal unions used in multiple places:
   ```ts
   type Status = 'pending' | 'active' | 'completed';
   ```
2. Repeated string literals across the codebase that represent the same concept
3. Type unions used as discriminators in objects

## Why enums are better

- Refactoring: rename in one place
- Autocomplete: IDE shows all options
- Runtime: can iterate over values
- Validation: can check if value is valid

## Exploration hints

- Check how the type is used across files
- Look for related constants or string literals
- Consider if the values are used at runtime

---

## Files

{{FILES}}
