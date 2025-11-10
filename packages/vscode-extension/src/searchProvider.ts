import * as vscode from 'vscode';
import { AnyUsageResult } from './anyFinder';
import { buildFolderTree, FolderNode, FileNode, getFolderIssueCount } from './treeBuilder';

export type ViewMode = 'list' | 'tree';

export class SearchResultProvider implements vscode.TreeDataProvider<SearchResultItem> {
  private results: AnyUsageResult[] = [];
  private _viewMode: ViewMode = 'list';

  private _onDidChangeTreeData = new vscode.EventEmitter<SearchResultItem | undefined | void>();
  readonly onDidChangeTreeData = this._onDidChangeTreeData.event;

  get viewMode(): ViewMode {
    return this._viewMode;
  }

  set viewMode(mode: ViewMode) {
    this._viewMode = mode;
    this._onDidChangeTreeData.fire();
  }

  setResults(results: AnyUsageResult[]) {
    this.results = results;
    this._onDidChangeTreeData.fire();
  }

  getResultCount(): number {
    return this.results.length;
  }

  getAllFolderItems(): FolderResultItem[] {
    if (this._viewMode !== 'tree') {
      return [];
    }

    const workspaceRoot = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath || '';
    const tree = buildFolderTree(this.results, workspaceRoot);
    const folders: FolderResultItem[] = [];

    const collectFolders = (map: Map<string, FolderNode | FileNode>) => {
      for (const node of map.values()) {
        if (node.type === 'folder') {
          folders.push(new FolderResultItem(node));
          collectFolders(node.children);
        }
      }
    };

    collectFolders(tree);
    return folders;
  }

  getTreeItem(element: SearchResultItem): vscode.TreeItem {
    return element;
  }

  getChildren(element?: SearchResultItem): Thenable<SearchResultItem[]> {
    if (!element) {
      if (this._viewMode === 'list') {
        return Promise.resolve(this.results.map(r => new LineResultItem(r)));
      } else {
        const workspaceRoot = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath || '';
        const tree = buildFolderTree(this.results, workspaceRoot);

        const items: SearchResultItem[] = [];
        for (const [name, node] of tree) {
          if (node.type === 'folder') {
            items.push(new FolderResultItem(node));
          } else {
            items.push(new FileResultItem(node.path, node.results));
          }
        }
        return Promise.resolve(items);
      }
    } else if (element instanceof FolderResultItem) {
      const items: SearchResultItem[] = [];
      for (const [name, node] of element.node.children) {
        if (node.type === 'folder') {
          items.push(new FolderResultItem(node));
        } else {
          items.push(new FileResultItem(node.path, node.results));
        }
      }
      return Promise.resolve(items);
    } else if (element instanceof FileResultItem) {
      return Promise.resolve(
        element.results.map(r => new LineResultItem(r))
      );
    }

    return Promise.resolve([]);
  }
}

class FolderResultItem extends vscode.TreeItem {
  constructor(public readonly node: FolderNode) {
    super(node.name, vscode.TreeItemCollapsibleState.Expanded);

    const count = getFolderIssueCount(node);
    this.description = `${count} ${count === 1 ? 'issue' : 'issues'}`;
    this.iconPath = new vscode.ThemeIcon('folder');
    this.contextValue = 'LinoNodeFolder';
  }
}

class FileResultItem extends vscode.TreeItem {
  constructor(
    public readonly filePath: string,
    public readonly results: AnyUsageResult[]
  ) {
    super(
      vscode.workspace.asRelativePath(filePath).split('/').pop() || filePath,
      vscode.TreeItemCollapsibleState.Collapsed
    );

    this.description = `${results.length} ${results.length === 1 ? 'issue' : 'issues'}`;
    this.iconPath = new vscode.ThemeIcon('file');
    this.contextValue = 'LinoNodeFile';
    this.resourceUri = vscode.Uri.file(filePath);
  }
}

class LineResultItem extends vscode.TreeItem {
  constructor(public readonly result: AnyUsageResult) {
    super(result.text, vscode.TreeItemCollapsibleState.None);

    this.description = `Ln ${result.line + 1}, Col ${result.column + 1}`;
    this.tooltip = result.text;

    this.command = {
      command: 'lino.openFile',
      title: 'Open File',
      arguments: [result.uri, result.line, result.column]
    };

    this.iconPath = new vscode.ThemeIcon(
      result.type === 'colonAny' ? 'symbol-variable' : 'symbol-keyword'
    );
    this.contextValue = 'LinoNodeIssue';
  }
}

type SearchResultItem = FolderResultItem | FileResultItem | LineResultItem;
