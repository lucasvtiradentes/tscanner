# Magic Numbers Detector

Find hardcoded numeric literals that should be extracted to named constants.

## What to flag

- Numbers used in comparisons (except 0, 1, -1)
- Array indices beyond 0 and 1
- Timeout/delay values
- Size limits, thresholds
- Port numbers, status codes

## What to ignore

- 0, 1, -1 in simple comparisons
- Numbers in tests
- Numbers with clear context from variable name

{{FILES}}
