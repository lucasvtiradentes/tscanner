---
"tscanner": patch
---

Drain script-rule and AI provider stdout/stderr while child processes run, preventing scans from hanging when external commands emit more output than the OS pipe buffer can hold.
