# Cache Implementation Guide

This document describes the caching system in tscanner for builtin rules, script rules, and AI rules.

## Overview

tscanner has three independent persistent caches:

| Cache | Purpose | Key | Invalidation |
|-------|---------|-----|--------------|
| FileCache | Builtin/regex rules | file path | file mtime + config_hash |
| ScriptCache | Script rules (.ts) | rule_name | script mtime + scanned files mtime |
| AiCache | AI rules (.md prompts) | rule_name | prompt mtime + scanned files mtime |

## Cache File Locations

All caches are stored in the system cache directory:

```
~/.cache/tscanner/
├── cache_{config_hash}.json          # FileCache
├── script_cache_{config_hash}.json   # ScriptCache
└── ai_cache_{config_hash}.json       # AiCache
```

The `config_hash` is computed from the entire config content (rules, patterns, options). When config changes, new cache files are created automatically.

## Cache Implementations

### 1. FileCache (`tscanner_cache/src/file_cache.rs`)

Used for builtin and regex rules. Caches per-file results.

```rust
struct FileCacheEntry {
    mtime: SystemTime,      // File modification time when cached
    config_hash: u64,       // Config hash when cached
    issues: Vec<Issue>,     // Cached issues for this file
}

// Key: PathBuf (file path)
// Cache hit: mtime matches AND config_hash matches
```

**Usage in Scanner:**
- `cache.get(path)` - returns cached issues if file hasn't changed
- `cache.insert(path, issues)` - stores issues for a file
- `cache.flush()` - persists to disk

### 2. ScriptCache (`tscanner_cache/src/script_cache.rs`)

Used for script rules. Caches per-rule results based on script file + scanned files mtimes.

```rust
struct ScriptCacheEntry {
    script_mtime: u64,                    // Script .ts file mtime
    files_mtimes: HashMap<PathBuf, u64>,  // Scanned files mtimes
    issues: Vec<Issue>,                   // Cached issues
}

// Key: String (rule_name)
// Cache hit: script_mtime matches AND all files_mtimes match
```

**Usage in ScriptExecutor:**
```rust
// Check cache
if let Some(cached) = self.cache.get(rule_name, &script_path, &files) {
    return Ok(cached);
}

// After execution, store in cache
self.cache.insert(rule_name, &script_path, &files, issues);
```

### 3. AiCache (`tscanner_cache/src/ai_cache.rs`)

Used for AI rules. Caches per-rule results based on prompt file + scanned files mtimes.

```rust
struct AiCacheEntry {
    prompt_mtime: u64,                    // Prompt .md file mtime
    files_mtimes: HashMap<PathBuf, u64>,  // Scanned files mtimes
    issues: Vec<Issue>,                   // Cached issues
}

// Key: String (rule_name)
// Cache hit: prompt_mtime matches AND all files_mtimes match
```

**Usage in AiExecutor:**
```rust
// Check cache
if let Some(cached) = self.cache.get(rule_name, &prompt_path, &files) {
    return Ok(cached);
}

// After execution, store in cache
self.cache.insert(rule_name, &prompt_path, &files, issues);
```

## Config Hash Computation

The `config_hash` is computed in `tscanner_scanner/src/config_ext.rs`:

```rust
fn compute_hash(&self) -> u64 {
    let mut hasher = DefaultHasher::new();

    // Hash file patterns
    for pattern in &self.files.include { pattern.hash(&mut hasher); }
    for pattern in &self.files.exclude { pattern.hash(&mut hasher); }

    // Hash builtin rules (name, severity, include, exclude, options)
    // Hash regex rules (name, pattern, include, exclude)
    // Hash script rules (name, command, include, exclude)
    // Hash AI rules (name, prompt path, include, exclude)

    hasher.finish()
}
```

Config changes → new config_hash → new cache files → effectively invalidates all caches.

## Scanner Integration

### Creating Caches

In `cmd_check` (CLI):

```rust
let (cache, ai_cache, script_cache) = if no_cache {
    // Empty caches - won't load from disk, won't persist
    (FileCache::new(), AiCache::new(), ScriptCache::new())
} else {
    // Load from disk using config_hash
    (
        FileCache::with_config_hash(config_hash),
        AiCache::with_config_hash(config_hash),
        ScriptCache::with_config_hash(config_hash),
    )
};

let scanner = Scanner::with_caches_and_config_dir(
    config,
    Arc::new(cache),
    Arc::new(ai_cache),
    Arc::new(script_cache),
    root,
    config_dir,
);
```

### Flushing Caches

At the end of scanning (in `codebase.rs`, `staged.rs`, `branch.rs`):

```rust
self.cache.flush();         // FileCache
self.ai_cache.flush();      // AiCache
self.script_cache.flush();  // ScriptCache
```

## Cache Invalidation Scenarios

| Change | FileCache | ScriptCache | AiCache |
|--------|-----------|-------------|---------|
| `config.jsonc` modified | ✓ New file | ✓ New file | ✓ New file |
| Source file modified | ✓ mtime check | ✓ mtime check | ✓ mtime check |
| Script rule `.ts` modified | - | ✓ mtime check | - |
| AI prompt `.md` modified | - | - | ✓ mtime check |
| `--no-cache` flag | ✓ Bypassed | ✓ Bypassed | ✓ Bypassed |

## Helper Function for mtime

All caches use the same mtime extraction:

```rust
fn get_mtime_secs(path: &Path) -> Option<u64> {
    fs::metadata(path)
        .ok()?
        .modified()
        .ok()?
        .duration_since(UNIX_EPOCH)
        .ok()
        .map(|d| d.as_secs())
}
```

## Performance Impact

Real-world measurements on this codebase:

| Scenario | Without Cache | With Cache |
|----------|---------------|------------|
| Regular rules (285 files) | 2.5s | 50ms |
| Script rules (8 rules) | 2.5s | 50ms |
| AI rules (2 rules, 199 files) | 1-2 min | 40ms |

## GitHub Action Considerations

For the GitHub Action package, consider:

1. **Cache Location**: GitHub Actions have ephemeral runners. Use `@actions/cache` to persist `~/.cache/tscanner/` between runs.

2. **Cache Key**: Use a combination of:
   - Config file hash
   - Workflow file hash (optional)
   - Date (for periodic cache refresh)

3. **Restore Keys**: Allow partial cache hits:
   ```yaml
   key: tscanner-cache-${{ hashFiles('.tscanner/config.jsonc') }}-${{ github.sha }}
   restore-keys: |
     tscanner-cache-${{ hashFiles('.tscanner/config.jsonc') }}-
     tscanner-cache-
   ```

4. **Cache Size**: Monitor cache size. The script_cache can be 80KB+ with many rules.

5. **PR vs Push**: Consider different cache strategies:
   - Push to main: Update cache
   - PR: Use cache but don't update (to avoid cache pollution)

## Code References

- Cache implementations: `packages/rust-core/crates/tscanner_cache/src/`
- Scanner integration: `packages/rust-core/crates/tscanner_scanner/src/scanner/core.rs`
- CLI cache creation: `packages/rust-core/crates/tscanner_cli/src/commands/check/command.rs`
- Config hash: `packages/rust-core/crates/tscanner_scanner/src/config_ext.rs`
