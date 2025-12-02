# Scanner Pipeline and Data Flow

## Scanner Structure

The `Scanner` struct orchestrates the entire analysis pipeline:

```rust
pub struct Scanner {
    registry: RuleRegistry,
    config: TscannerConfig,
    cache: FileCache,
}
```

**Initialization:**
- `Scanner::new(config)` - Creates scanner with empty cache
- `Scanner::with_cache(config, cache)` - Creates scanner with pre-loaded cache

## Scan Flow

### 1. File Discovery

Uses `WalkBuilder` with gitignore support and config patterns:

```
WalkBuilder::new(root)
  ├─ Compile config.include patterns into GlobSet
  ├─ Compile config.exclude patterns into GlobSet
  ├─ Filter directories by exclude patterns
  ├─ Filter files: include.is_match(path) && !exclude.is_match(path)
  └─ Optional: Filter by changed files (branch mode)
```

All pattern matching uses **relative paths** from root for consistency.

### 2. Parallel Processing

```
files.par_iter().flat_map(|path| {
  ├─ Check cache (mtime + config_hash)
  ├─ If cache hit: return cached issues
  └─ If cache miss: analyze_file()
})
```

### 3. File Analysis

```rust
analyze_file(path):
  1. Read file content
  2. Parse DisableDirectives
     - // tscanner-disable-file
     - // tscanner-disable-line rule-name
     - // tscanner-disable-next-line rule-name
  3. Parse with SWC (parse_file)
  4. Get enabled rules for file (registry.get_enabled_rules)
  5. Run each rule's check() method
  6. Filter issues by disable directives
  7. Cache results (path, mtime, config_hash, issues)
```

### 4. Post-processing

Branch mode filtering:
```
For each issue:
  ├─ Get modified line ranges from git diff
  └─ Keep only issues on modified lines
```

## Scan Methods

### Full Workspace Scan
```rust
scan(&self, root: &Path, file_filter: Option<Vec<PathBuf>>) → Vec<Issue>
```
- Walks directory tree
- Applies gitignore rules
- Parallelizes with Rayon
- Uses cache

### Single File Scan
```rust
scan_single(&self, path: &Path) → Vec<Issue>
```
- Invalidates cache entry first
- Analyzes single file
- Updates cache

### Content Scan
```rust
scan_content(&self, path: &Path, content: &str) → Vec<Issue>
```
- In-memory analysis
- No cache interaction
- Used for unsaved changes

## Branch Mode Integration

### Changed Files Detection
```bash
git diff --name-only target_branch
```

Returns list of modified file paths.

### Modified Line Ranges
```bash
git diff -U0 target_branch -- file.ts
```

Parse hunk headers:
```
@@ -10,3 +10,5 @@
   ↓      ↓
  old    new (start, count)
```

Extract modified line ranges from new side.

### Issue Filtering
```rust
For each issue:
  if issue.line in modified_ranges:
    keep
  else:
    discard
```

## Disable Directives

### File-level
```typescript
// tscanner-disable-file
```
Skips all rules for entire file.

### Line-level
```typescript
const x: any = 1; // tscanner-disable-line no-explicit-any
```
Disables specific rule for current line.

### Next-line
```typescript
// tscanner-disable-next-line no-explicit-any
const x: any = 1;
```
Disables specific rule for next line.

## Glob Pattern Matching

### Pattern Semantics

TScanner uses `globset` crate for fast pattern matching. All paths are normalized to **relative paths** before matching.

**Global patterns** (config root level):
```json
{
  "include": ["**/*.ts", "**/*.tsx"],
  "exclude": ["**/node_modules/**", "**/dist/**"]
}
```

**Rule-specific patterns** (per-rule):
```json
{
  "builtinRules": {
    "no-explicit-any": {
      "include": ["src/**"],
      "exclude": ["**/*.test.ts"]
    }
  }
}
```

### Intersection Logic

Rule patterns **intersect** with global patterns (not replace):

```
file_matches = global_include.match(path)
            && !global_exclude.match(path)
            && (rule_include.match(path) OR rule_include.is_empty())
            && (!rule_exclude.match(path) OR rule_exclude.is_empty())
```

| Scenario | Behavior |
|----------|----------|
| Global only | Uses global include/exclude |
| Rule include set | Must match BOTH global AND rule include |
| Rule exclude set | Excluded if matches global OR rule exclude |
| Empty rule patterns | Falls back to global patterns |

### CompiledRuleConfig

Each rule compiles to:

```rust
pub struct CompiledRuleConfig {
    pub enabled: bool,
    pub severity: Severity,
    pub global_include: GlobSet,
    pub global_exclude: GlobSet,
    pub rule_include: Option<GlobSet>,
    pub rule_exclude: Option<GlobSet>,
}
```

## Performance Optimizations

1. **Parallel Processing** - Rayon par_iter for multi-core utilization
2. **Cache Hits** - Skip parsing/analysis for unchanged files
3. **Early Exit** - File-level disable directive skips entire analysis
4. **Gitignore** - WalkBuilder filters out ignored files upfront
5. **GlobSet** - Pre-compiled patterns for O(n) matching

## Related Documentation

- [Caching](04-caching.md) - Cache invalidation and persistence
- [Rule System](02-rule-system.md) - How rules are registered and executed
