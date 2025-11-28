# PR Comment System

## Overview

The GitHub Action posts scan results as PR comments. It uses an idempotent update strategy with bot marker detection to maintain a single comment per PR, updated on each commit.

## Comment Structure

### No Issues

```markdown
<!-- tscanner-pr-comment -->
## ✅ TScanner - No Issues Found

**Issues:** 0
**Mode:** codebase

All files passed validation!

---
**Last updated:** 01/28/2025, 10:30:45 (UTC-3)
**Last commit analyzed:** `a1b2c3d` - feat: add validation
```

### Issues Found

```markdown
<!-- tscanner-pr-comment -->
## ❌ TScanner - Errors Found

**Issues:** 15 (10 errors, 5 warnings)
**Mode:** branch (main)

---

<div align="center">

<details>
<summary><strong>📋 Issues grouped by rule (3)</strong></summary>
<br />

<div align="left">

[collapsible rule groups...]

</div></details>

</div>

---

<div align="center">

<details>
<summary><strong>📁 Issues grouped by file (5)</strong></summary>
<br />

<div align="left">

[collapsible file groups...]

</div></details>

</div>

---
**Last updated:** 01/28/2025, 10:30:45 (UTC-3)
**Last commit analyzed:** `a1b2c3d` - feat: add validation
```

## Markdown Generation

### Issue Formatting

**Grouped by Rule View**

```typescript
<details>
<summary>✗ <strong>no-console</strong> - 5 issues - 3 files</summary>
<br />

<strong>src/index.ts</strong> - 2 issues

- <a href="https://github.com/owner/repo/pull/123/files#diff-abc123R15">15:5</a> - <code>console.log('debug')</code>
- <a href="https://github.com/owner/repo/pull/123/files#diff-abc123R28">28:3</a> - <code>console.error('fail')</code>

</details>
```

**Grouped by File View**

```typescript
<details>
<summary><strong>src/index.ts</strong> - 3 issues - 2 rules</summary>
<br />

<strong>no-console</strong> - 2 issues

- <a href="https://github.com/owner/repo/pull/123/files#diff-abc123R15">15:5</a> - <code>console.log('debug')</code>
- <a href="https://github.com/owner/repo/pull/123/files#diff-abc123R28">28:3</a> - <code>console.error('fail')</code>

<strong>unused-variable</strong> - 1 issue

- <a href="https://github.com/owner/repo/pull/123/files#diff-abc123R42">42:7</a> - <code>const unused = 123</code>

</details>
```

### URL Construction

```typescript
function buildPrFileUrl(
  owner: string,
  repo: string,
  prNumber: number,
  filePath: string,
  line: number
): string {
  const fileHash = createHash('sha256')
    .update(filePath)
    .digest('hex');

  return `https://github.com/${owner}/${repo}/pull/${prNumber}/files#diff-${fileHash}R${line}`;
}
```

Links navigate directly to specific lines in PR file diff view.

### HTML Escaping

```typescript
function escapeHtml(text: string): string {
  return text
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
    .replace(/'/g, '&#039;');
}
```

Prevents XSS when rendering code snippets in markdown.

### Collapsible Sections

Both grouping views use `<details>` tags:
- Default collapsed to reduce visual noise
- Summary shows key metrics (issue count, file/rule count)
- Nested structure for multi-level organization

## Update vs Create Logic

### Bot Detection

```typescript
const COMMENT_MARKER = '<!-- tscanner-pr-comment -->';

const botComment = existingComments.data.find(
  (c) => c.user?.type === 'Bot' && c.body?.includes(COMMENT_MARKER)
);
```

**Detection Strategy:**
1. Check user type is `Bot`
2. Verify comment contains HTML marker
3. Return first match (only one TScanner comment expected)

### Idempotent Updates

```typescript
if (botComment) {
  await octokit.rest.issues.updateComment({
    owner,
    repo,
    comment_id: botComment.id,
    body: comment,
  });
  githubHelper.logInfo(`Updated existing comment #${botComment.id}`);
} else {
  await octokit.rest.issues.createComment({
    owner,
    repo,
    issue_number: prNumber,
    body: comment,
  });
  githubHelper.logInfo('Created new comment');
}
```

**Update Path:**
- Finds existing TScanner comment by marker
- Replaces entire comment body
- Preserves comment ID and position in thread

**Create Path:**
- No existing comment found
- Creates new comment at end of thread
- Subsequent runs will update this comment

### Commit Tracking

```typescript
const latestCommitSha = prInfo.head.sha.substring(0, 7);
const commitMessage = await getCommitMessageFromApi(octokit, owner, repo, prInfo.head.sha);
```

Each update displays:
- Short SHA (7 chars)
- First line of commit message
- Timestamp with configured timezone

## GitHub API Usage

### Dependencies

```typescript
import { getOctokit } from '@actions/github';
import type { Octokit } from '@octokit/rest';
```

### API Calls

**List Comments**
```typescript
const existingComments = await octokit.rest.issues.listComments({
  owner,
  repo,
  issue_number: prNumber,
});
```

**Update Comment**
```typescript
await octokit.rest.issues.updateComment({
  owner,
  repo,
  comment_id: botComment.id,
  body: comment,
});
```

**Create Comment**
```typescript
await octokit.rest.issues.createComment({
  owner,
  repo,
  issue_number: prNumber,
  body: comment,
});
```

**Get Commit**
```typescript
const { data } = await octokit.rest.repos.getCommit({
  owner,
  repo,
  ref: sha,
});
const commitMessage = data.commit.message.split('\n')[0];
```

### Authentication

```typescript
const octokit = githubHelper.getOctokit(inputs.token);
```

Uses `GITHUB_TOKEN` secret with `pull_request` write permissions.

## Summary Statistics

### Calculation

```typescript
const buildIssuesSummary = () => {
  if (totalErrors > 0 && totalWarnings > 0) {
    return `${totalIssues} (${totalErrors} errors, ${totalWarnings} warnings)`;
  }
  if (totalErrors > 0) {
    return `${totalErrors} (${totalErrors} errors)`;
  }
  if (totalWarnings > 0) {
    return `${totalWarnings} (${totalWarnings} warnings)`;
  }
  return '0';
};
```

**Display Logic:**
- Mixed severity: Show total with breakdown
- Single severity: Show count with label
- Zero issues: Show '0'

### Scan Mode Label

```typescript
const buildModeLabel = () => {
  return targetBranch
    ? `branch (${targetBranch})`
    : 'codebase';
};
```

Indicates if scanning:
- `branch (main)` - Only changed files vs target branch
- `codebase` - Entire repository

## Edge Cases

### No Issues Found

```typescript
if (totalIssues === 0) {
  return `<!-- tscanner-pr-comment -->
## ✅ TScanner - No Issues Found

**Issues:** 0
**Mode:** ${buildModeLabel()}

All files passed validation!
[...]`;
}
```

Posts positive feedback comment instead of staying silent.

### Too Many Issues

No artificial limits. GitHub comments support up to 65,536 characters. Collapsible sections keep UI manageable regardless of count.

### Failed Commit Message Fetch

```typescript
try {
  const { data } = await octokit.rest.repos.getCommit({ owner, repo, ref: sha });
  return data.commit.message.split('\n')[0];
} catch {
  githubHelper.logWarning(`Failed to get commit message for ${sha}`);
  return '';
}
```

Falls back to showing only SHA without message.

### No Pull Request Context

```typescript
if (!prInfo) {
  return;
}
```

Skips comment logic entirely when not in `pull_request` event context.

### Timestamp Formatting Error

```typescript
try {
  const formatted = now.toLocaleString('en-US', {
    timeZone: timezone,
    [...]
  });
  const offset = getTimezoneOffset(timezone);
  return `${formatted} (UTC${offset})`;
} catch {
  return `${now.toISOString()} (UTC)`;
}
```

Falls back to ISO 8601 format with UTC timezone.

## Data Flow

```
1. index.ts → Get PR context from GitHub
2. index.ts → Execute scan with scanner.ts
3. index.ts → Fetch commit message via Octokit
4. index.ts → Call updateOrCreateComment()
5. comment-updater.ts → Build markdown body
6. comment-updater.ts → List existing PR comments
7. comment-updater.ts → Find bot comment by marker
8. comment-updater.ts → Update or create comment
9. GitHub → Render markdown in PR thread
```

## Type Definitions

```typescript
type CommentUpdateParams = {
  octokit: Octokit;
  owner: string;
  repo: string;
  prNumber: number;
  scanResult: ScanResult;
  timezone: string;
  commitSha: string;
  commitMessage: string;
  targetBranch?: string;
};

type ScanResult = {
  totalIssues: number;
  totalErrors: number;
  totalWarnings: number;
  totalFiles: number;
  totalRules: number;
  groupBy: GroupMode;
  ruleGroups: RuleGroup[];
  ruleGroupsByRule: RuleGroup[];
};

type RuleGroup = {
  ruleName: string;
  severity: Severity;
  issueCount: number;
  fileCount: number;
  files: FileIssues[];
};

type FileIssues = {
  filePath: string;
  issues: Issue[];
};

type Issue = {
  line: number;
  column: number;
  message: string;
  lineText: string;
  ruleName?: string;
};
```
