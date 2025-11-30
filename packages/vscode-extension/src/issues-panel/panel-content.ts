import * as vscode from 'vscode';
import { GroupMode, ViewMode, getCurrentWorkspaceFolder } from '../common/lib/vscode-utils';
import { type IssueResult, NodeKind } from '../common/types';
import { logger } from '../common/utils/logger';
import { buildFolderTree } from './utils/tree-builder';
import { FileResultItem, FolderResultItem, LineResultItem, RuleGroupItem } from './utils/tree-items';

type PanelContentItem = RuleGroupItem | FolderResultItem | FileResultItem | LineResultItem;

export class IssuesPanelContent implements vscode.TreeDataProvider<PanelContentItem> {
  private results: IssueResult[] = [];
  private _viewMode: ViewMode = ViewMode.List;
  private _groupMode: GroupMode = GroupMode.Default;

  private _onDidChangeTreeData = new vscode.EventEmitter<PanelContentItem | undefined>();
  readonly onDidChangeTreeData = this._onDidChangeTreeData.event;

  get viewMode(): ViewMode {
    return this._viewMode;
  }

  set viewMode(mode: ViewMode) {
    this._viewMode = mode;
    this._onDidChangeTreeData.fire(undefined);
  }

  get groupMode(): GroupMode {
    return this._groupMode;
  }

  set groupMode(mode: GroupMode) {
    this._groupMode = mode;
    this._onDidChangeTreeData.fire(undefined);
  }

  setResults(results: IssueResult[]) {
    this.results = results;
    this._onDidChangeTreeData.fire(undefined);
  }

  getResults(): IssueResult[] {
    return this.results;
  }

  getResultCount(): number {
    return this.results.length;
  }

  private groupByRule(): Map<string, IssueResult[]> {
    const grouped = new Map<string, IssueResult[]>();

    for (const result of this.results) {
      const rule = result.rule || 'unknown';
      if (!grouped.has(rule)) {
        grouped.set(rule, []);
      }
      grouped.get(rule)?.push(result);
    }

    return grouped;
  }

  getAllFolderItems(): FolderResultItem[] {
    if (this._viewMode !== ViewMode.Tree) {
      return [];
    }

    const workspaceRoot = getCurrentWorkspaceFolder()?.uri.fsPath || '';

    const tree = buildFolderTree(this.results, workspaceRoot);
    const folders: FolderResultItem[] = [];

    const collectFolders = (map: Map<string, any>) => {
      if (!map || typeof map !== 'object') {
        logger.error(`Invalid map passed to collectFolders: ${typeof map}`);
        return;
      }

      for (const node of map.values()) {
        if (node.type === NodeKind.Folder) {
          folders.push(new FolderResultItem(node));
          collectFolders(node.children);
        }
      }
    };

    collectFolders(tree);
    return folders;
  }

  getTreeItem(element: PanelContentItem): vscode.TreeItem {
    return element;
  }

  getChildren(element?: PanelContentItem): Thenable<PanelContentItem[]> {
    if (!element) {
      if (this._groupMode === GroupMode.Rule) {
        const grouped = this.groupByRule();
        const sortedEntries = Array.from(grouped.entries()).sort((a, b) => a[0].localeCompare(b[0]));
        return Promise.resolve(
          sortedEntries.map(([rule, results]) => new RuleGroupItem(rule, results, this._viewMode)),
        );
      }

      if (this._viewMode === ViewMode.List) {
        const grouped = new Map<string, IssueResult[]>();
        for (const result of this.results) {
          const filePath = result.uri.fsPath;
          if (!grouped.has(filePath)) {
            grouped.set(filePath, []);
          }
          grouped.get(filePath)?.push(result);
        }

        const sortedEntries = Array.from(grouped.entries()).sort((a, b) => a[0].localeCompare(b[0]));
        return Promise.resolve(sortedEntries.map(([path, results]) => new FileResultItem(path, results)));
      }

      const workspaceRoot = getCurrentWorkspaceFolder()?.uri.fsPath || '';
      const tree = buildFolderTree(this.results, workspaceRoot);

      const items: PanelContentItem[] = [];
      for (const [, node] of tree) {
        if (node.type === NodeKind.Folder) {
          items.push(new FolderResultItem(node));
        } else {
          items.push(new FileResultItem(node.path, node.results));
        }
      }
      return Promise.resolve(items);
    }
    if (element instanceof RuleGroupItem) {
      if (element.viewMode === ViewMode.List) {
        const sortedResults = [...element.results].sort((a, b) => {
          const pathCompare = a.uri.fsPath.localeCompare(b.uri.fsPath);
          if (pathCompare !== 0) return pathCompare;
          return a.line - b.line;
        });
        return Promise.resolve(sortedResults.map((r) => new LineResultItem(r)));
      }
      const workspaceRoot = getCurrentWorkspaceFolder()?.uri.fsPath || '';
      const tree = buildFolderTree(element.results, workspaceRoot);

      const items: PanelContentItem[] = [];
      for (const [, node] of tree) {
        if (node.type === NodeKind.Folder) {
          items.push(new FolderResultItem(node));
        } else {
          items.push(new FileResultItem(node.path, node.results));
        }
      }
      return Promise.resolve(items);
    }
    if (element instanceof FolderResultItem) {
      const items: PanelContentItem[] = [];
      for (const [, node] of element.node.children) {
        if (node.type === NodeKind.Folder) {
          items.push(new FolderResultItem(node));
        } else {
          items.push(new FileResultItem(node.path, node.results));
        }
      }
      return Promise.resolve(items);
    }
    if (element instanceof FileResultItem) {
      const sortedResults = [...element.results].sort((a, b) => a.line - b.line);
      return Promise.resolve(sortedResults.map((r) => new LineResultItem(r)));
    }

    return Promise.resolve([]);
  }
}
