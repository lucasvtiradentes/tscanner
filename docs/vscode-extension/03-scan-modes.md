# Scan Modes

TScanner supports two scanning modes: Codebase and Branch.

## Mode Comparison

| Aspect | Codebase | Branch |
|--------|----------|--------|
| **Scope** | All matching files | Only changed files |
| **Use case** | Audit entire project | PR validation |
| **Speed** | Slower on large codebases | Fast (fewer files) |
| **Filtering** | None | Issues in modified lines only |

## Codebase Mode

Scans all files matching include/exclude patterns.

```
User triggers scan
       │
       ▼
Load config (.tscanner/config.jsonc)
       │
       ▼
Find all *.ts/*.tsx files
       │
       ▼
Apply include/exclude patterns
       │
       ▼
Send to Rust server → Parse → Run rules
       │
       ▼
Display all issues in TreeView
```

## Branch Mode

Scans only files changed vs a target branch, filtering issues to modified lines.

```
User triggers scan (Branch mode)
       │
       ▼
Get changed files vs target branch
       │
       ├──► VSCode Git API: repo.diffWithHEAD()
       │    (uncommitted changes)
       │
       └──► VSCode Git API: repo.diffBetween(target, HEAD)
            (committed changes on branch)
       │
       ▼
Merge file lists → Filter to *.ts/*.tsx
       │
       ▼
Send filtered files to Rust server
       │
       ▼
For each file with issues:
       │
       ├──► Get modified line ranges (git diff hunks)
       │
       └──► Filter issues to modified lines only
       │
       ▼
Display filtered issues
```

## Git Diff Integration

### Getting Changed Files

Uses VSCode Git extension API:

```typescript
export async function getChangedFiles(
  workspaceRoot: string,
  compareBranch: string
): Promise<Set<string>> {
  const repo = getRepository(workspaceRoot);

  const uncommitted = await repo.diffWithHEAD();
  const committed = await repo.diffBetween(compareBranch, currentBranch);

  return new Set([
    ...uncommitted.map(c => relativePath(c.uri)),
    ...committed.map(c => relativePath(c.uri)),
  ]);
}
```

### Getting Modified Line Ranges

Parses git diff hunks to extract added line numbers:

```typescript
export async function getModifiedLineRanges(
  workspaceRoot: string,
  filePath: string,
  compareBranch: string
): Promise<ModifiedLineRange[]> {
  const diff = execSync(
    `git diff "${compareBranch}...${currentBranch}" -- "${filePath}"`,
    { cwd: workspaceRoot }
  );

  // Parse hunk headers: @@ -10,5 +10,7 @@
  // Track added lines (lines starting with +)
  // Return ranges: [{ startLine: 10, lineCount: 7 }]
}
```

**Hunk parsing:**

```
@@ -10,5 +10,7 @@           ← Hunk header
 unchanged line              ← Context (skip)
-removed line                ← Removed (skip)
+added line 1                ← Track as line 10
+added line 2                ← Track as line 11
 unchanged line              ← Context (skip)
```

### Filtering Issues

Only issues on modified lines are shown:

```typescript
function getNewIssues(
  allIssues: IssueResult[],
  modifiedRanges: Map<string, ModifiedLineRange[]>
): IssueResult[] {
  return allIssues.filter(issue => {
    const ranges = modifiedRanges.get(issue.uri.fsPath);
    if (!ranges) return false;

    return ranges.some(range =>
      issue.line >= range.startLine &&
      issue.line < range.startLine + range.lineCount
    );
  });
}
```

## 30-Second TTL Cache

Git diff results are cached to avoid repeated git commands:

```typescript
const changedFilesCache: Map<string, Set<string>> = new Map();
const lastCacheUpdate: Map<string, number> = new Map();
const CACHE_TTL = 30000; // 30 seconds

export async function getChangedFiles(workspaceRoot: string, compareBranch: string) {
  const cacheKey = `${workspaceRoot}:${compareBranch}`;
  const now = Date.now();
  const lastUpdate = lastCacheUpdate.get(cacheKey) || 0;

  if (now - lastUpdate < CACHE_TTL && changedFilesCache.has(cacheKey)) {
    return changedFilesCache.get(cacheKey)!;
  }

  // Fetch fresh data...
  changedFilesCache.set(cacheKey, result);
  lastCacheUpdate.set(cacheKey, now);
  return result;
}
```

**Cache invalidation:**
- On file save in branch mode
- On explicit hard scan
- After TTL expires

```typescript
export function invalidateCache(workspaceRoot?: string) {
  if (workspaceRoot) {
    // Clear entries for specific workspace
  } else {
    changedFilesCache.clear();
    lastCacheUpdate.clear();
  }
}
```

## File Watcher Integration

Single-file updates respect scan mode:

```typescript
const updateSingleFile = async (uri: vscode.Uri) => {
  if (currentScanMode === ScanMode.Branch) {
    invalidateCache();
    const changedFiles = await getChangedFiles(workspaceRoot, compareBranch);

    if (!changedFiles.has(relativePath)) {
      return; // File not in changed set, skip
    }

    // Scan file, then filter issues to modified lines
    const ranges = await getModifiedLineRanges(workspaceRoot, relativePath, compareBranch);
    newResults = filterToModifiedLines(newResults, ranges);
  }

  // Merge with existing results
  panelContent.setResults([...filtered, ...newResults]);
};
```

## Mode Switching

Via status bar click → Settings Menu → Scan Mode:

```typescript
async function selectScanMode(context: ExtensionContext) {
  const items = [
    { label: 'Codebase', description: 'Scan all files' },
    { label: 'Branch', description: 'Scan changed files only' },
  ];

  const selected = await vscode.window.showQuickPick(items);

  if (selected?.label === 'Branch') {
    const branch = await selectTargetBranch();
    setWorkspaceState(context, WorkspaceStateKey.CompareBranch, branch);
  }

  setWorkspaceState(context, WorkspaceStateKey.ScanMode, selected?.label);
  executeCommand(Command.FindIssue); // Re-scan
}
```
