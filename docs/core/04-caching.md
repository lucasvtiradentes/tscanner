# Caching Strategy

## Two-Tier Cache Architecture

**Memory Cache (Hot Path):**
- `DashMap<PathBuf, CacheEntry>` - Concurrent hash map
- Thread-safe access during parallel scanning
- Primary lookup layer

**Disk Cache (Persistence):**
- Location: `~/.cache/tscanner/cache_{config_hash}.json`
- Survives server restarts
- Loaded on initialization if config hash matches

## Cache Entry Structure

```rust
struct CacheEntry {
    mtime: SystemTime,
    config_hash: u64,
    issues: Vec<Issue>,
}
```

## Cache Validation

**Cache Key:**
- File path (canonical)

**Validity Conditions:**
```
is_valid = (entry.mtime == current_mtime) && (entry.config_hash == current_config_hash)
```

## Config Hash Computation

Deterministic hash of configuration state:

```rust
fn compute_hash(config: &Config) -> u64 {
    // BTreeMap ensures consistent ordering
    let mut rules: BTreeMap<String, _> = BTreeMap::new();

    for (name, rule) in &config.rules {
        if rule.enabled {
            rules.insert(name.clone(), (
                &rule.pattern,
                &rule.severity,
                &rule.include,
                &rule.exclude,
            ));
        }
    }

    hash(&rules)
}
```

**Invalidation:** Any config change produces new hash, invalidating all cached entries.

## Cache Operations

### get(path)
1. Check memory cache
2. Verify mtime matches filesystem
3. Verify config_hash matches current
4. Return cached issues if valid, else None

### insert(path, issues)
1. Get current mtime from filesystem
2. Store entry with current config_hash
3. Memory-only (disk flush happens separately)

### invalidate(path)
Remove single entry from memory cache

### clear()
1. Remove all memory entries
2. Delete disk cache file
3. Called on config changes

### flush()
1. Serialize memory cache to JSON
2. Write to `~/.cache/tscanner/cache_{config_hash}.json`
3. Called after each scan completes

## Disk Persistence Format

```json
[
  ["/path/to/file.ts", {
    "mtime": {"secs_since_epoch": 1234567890, "nanos_since_epoch": 0},
    "config_hash": 12345678901234567890,
    "issues": [...]
  }],
  ...
]
```

## Invalidation Triggers

| Trigger | Scope | Mechanism |
|---------|-------|-----------|
| File modified | Single file | mtime check fails |
| Config changed | All files | config_hash mismatch |
| Manual clear | All files | RPC `clear_cache` method |
| Single file scan | Single file | Explicit invalidation before scan |
| Git operations | Affected files | Extension invalidates via RPC |

## Performance Impact

**Cache Hit:**
- Skip file read
- Skip AST parsing
- Skip rule execution
- ~100x faster than full analysis

**Cache Miss:**
- Full analysis required
- Result stored for future hits

**Parallel Safety:**
- DashMap allows concurrent reads/writes
- No locks needed for cache access

## See Also

- [Scanner Flow](03-scanner-flow.md) - How scanner uses cache
