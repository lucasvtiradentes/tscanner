---
"tscanner": major
---

Store AI provider settings in project-local `.tscanner/local.jsonc` instead of global user config. `tscanner ai set/show/unset` now require a TScanner project and read/write the local file shared by CLI and VS Code AI scans.
