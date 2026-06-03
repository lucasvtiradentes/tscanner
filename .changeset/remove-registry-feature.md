---
'tscanner': minor
---

Remove the community-rules registry feature. The `tscanner registry` subcommand, the `registry/` folder, and all related fetcher/installer code are gone. Custom rules are now defined directly in `.tscanner/` (regex/script/ai) without an external registry.
