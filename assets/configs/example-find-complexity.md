# Find Complex Functions

Analyze the provided code and identify functions that are overly complex.

## What to look for

1. Functions with high cyclomatic complexity (many branches/loops)
2. Deeply nested code blocks (3+ levels)
3. Functions doing too many things (violating single responsibility)
4. Long parameter lists that should be objects

## Output format

Report each complex function with:
- The function name
- Why it's complex
- A brief suggestion for improvement

{{FILES}}
