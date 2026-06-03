---
"tscanner": major
"tscanner-common": major
---

Run script rules as normal commands with workspace root, `--files`, and `--options` arguments instead of sending input through stdin.

Script rules should now print `{ "issues": [...] }` to stdout. The script helper API now exposes `readScriptArgs` and `ScriptArgs` for the new argument-based format.
