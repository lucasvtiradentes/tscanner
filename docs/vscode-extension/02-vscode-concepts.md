# VSCode Concepts

How the extension uses VSCode APIs and concepts.

## Extension Storage

VSCode provides storage locations per-extension:

| Location | Usage | API |
|----------|-------|-----|
| `globalStorage` | User-wide settings, logs | `context.globalStorageUri` |
| `workspaceState` | Per-workspace settings | `context.workspaceState` |

**Workspace state keys used:**

```typescript
enum WorkspaceStateKey {
  ViewMode = 'viewMode',           // 'list' | 'tree'
  GroupMode = 'groupMode',         // 'default' | 'rule'
  ScanMode = 'scanMode',           // 'codebase' | 'branch'
  CompareBranch = 'compareBranch', // 'origin/main'
  CachedResults = 'cachedResults', // Serialized issues
  CustomConfigDir = 'customConfigDir',
}
```

## Debugging the Extension

**Launch configuration (`.vscode/launch.json`):**

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "name": "Run Extension",
      "type": "extensionHost",
      "request": "launch",
      "args": ["--extensionDevelopmentPath=${workspaceFolder}/packages/vscode-extension"]
    }
  ]
}
```

**Workflow:**
1. Open monorepo root in VSCode
2. Press F5 to launch Extension Development Host
3. New window opens with extension loaded
4. Set breakpoints in `src/` files
5. Debug Console shows extension logs

## UI Components

### StatusBar

Shows current scan mode and config status.

```typescript
class StatusBarManager {
  private statusBarItem: vscode.StatusBarItem;

  constructor() {
    this.statusBarItem = vscode.window.createStatusBarItem(
      vscode.StatusBarAlignment.Left,
      100
    );
    this.statusBarItem.command = 'tscanner.openSettingsMenu';
  }

  async update() {
    this.statusBarItem.text = hasConfig
      ? `$(shield) ${modeText}`  // "$(shield) Codebase" or "$(shield) Branch (main)"
      : '$(warning) [No rules]';
    this.statusBarItem.show();
  }
}
```

**States:**
- `$(shield) Codebase` - Full scan mode, config exists
- `$(shield) Branch (origin/main)` - Branch scan mode
- `$(warning) [No rules]` - No config found

### TreeView

Issues panel in activity bar sidebar.

```typescript
const treeView = vscode.window.createTreeView('tscannerExplorer', {
  treeDataProvider: panelContent,
});
```

**TreeDataProvider implementation:**

```typescript
class IssuesPanelContent implements vscode.TreeDataProvider<TreeNode> {
  private _onDidChangeTreeData = new vscode.EventEmitter<TreeNode | undefined>();
  readonly onDidChangeTreeData = this._onDidChangeTreeData.event;

  getTreeItem(element: TreeNode): vscode.TreeItem { ... }
  getChildren(element?: TreeNode): TreeNode[] { ... }

  setResults(results: IssueResult[]) {
    this.results = results;
    this._onDidChangeTreeData.fire(undefined); // Refresh entire tree
  }
}
```

**Node types:**
- `TscannerNodeFolder` - Directory grouping
- `TscannerNodeFile` - File grouping
- `TscannerNodeRuleGroup` - Rule grouping
- `TscannerNodeIssue` - Individual issue

### QuickPick

Settings menu for quick configuration.

```typescript
async function openSettingsMenu() {
  const items: vscode.QuickPickItem[] = [
    { label: '$(gear) Manage Rules', description: 'Enable/disable rules' },
    { label: '$(git-branch) Scan Mode', description: 'Codebase or Branch' },
    { label: '$(folder) Config Location', description: 'Change config path' },
  ];

  const selected = await vscode.window.showQuickPick(items, {
    placeHolder: 'TScanner Settings',
  });
}
```

### Context Keys

Control command visibility via context keys:

```typescript
enum ContextKey {
  ViewMode = 'tscannerViewMode',
  GroupMode = 'tscannerGroupMode',
  ScanMode = 'tscannerScanMode',
  Searching = 'tscannerSearching',
}

vscode.commands.executeCommand('setContext', 'tscannerSearching', true);
```

**Usage in package.json:**

```json
{
  "menus": {
    "view/title": [
      {
        "command": "tscanner.refresh",
        "when": "view == tscannerExplorer && !tscannerSearching"
      }
    ]
  }
}
```

## Disposables Pattern

All VSCode resources must be disposed on deactivation:

```typescript
export function activate(context: vscode.ExtensionContext) {
  const fileWatcher = vscode.workspace.createFileSystemWatcher('**/*.ts');
  const statusBarItem = vscode.window.createStatusBarItem();

  context.subscriptions.push(
    fileWatcher,
    statusBarItem,
    vscode.commands.registerCommand('tscanner.scan', scanWorkspace)
  );
}

export function deactivate() {
  disposeScanner(); // Kill Rust process
}
```

## File System Watcher

Monitor file changes for incremental updates:

```typescript
const fileWatcher = vscode.workspace.createFileSystemWatcher('**/*.{ts,tsx,js,jsx}');

fileWatcher.onDidChange(async (uri) => {
  await updateSingleFile(uri);
});

fileWatcher.onDidCreate(async (uri) => {
  await updateSingleFile(uri);
});

fileWatcher.onDidDelete(async (uri) => {
  removeFileFromResults(uri);
});
```

## Git Extension Integration

Access VSCode's built-in Git extension:

```typescript
type GitExtension = {
  getAPI(version: 1): GitAPI;
};

function getGitAPI(): GitAPI | null {
  const gitExtension = vscode.extensions.getExtension<GitExtension>('vscode.git');
  if (gitExtension?.isActive) {
    return gitExtension.exports.getAPI(1);
  }
  return null;
}
```

**Repository operations:**
- `repo.state.HEAD` - Current branch info
- `repo.diffWithHEAD()` - Uncommitted changes
- `repo.diffBetween(ref1, ref2)` - Changes between branches
