---
"tscanner": major
---

Remove VS Code editor settings from `.tscanner/config.jsonc`. Configure startup and auto-scan behavior through VS Code settings instead:

- `codeEditor.startupScan` -> `tscanner.scan.startup`
- `codeEditor.startupAiScan` -> `tscanner.aiScan.startup`
- `codeEditor.autoScanInterval` -> `tscanner.scan.autoInterval`
- `codeEditor.autoAiScanInterval` -> `tscanner.aiScan.autoInterval`
