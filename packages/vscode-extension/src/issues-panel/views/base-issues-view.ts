import { GroupMode, ViewMode } from 'tscanner-common';
import * as vscode from 'vscode';
import { getCurrentWorkspaceFolder } from '../../common/lib/vscode-utils';
import { type IssueResult, NodeKind } from '../../common/types';
import { buildFolderTree } from '../components/tree-builder';
import {
  ErrorMessageItem,
  FileResultItem,
  FolderResultItem,
  LineResultItem,
  RuleGroupItem,
} from '../components/tree-items';

type IssuesViewItem = RuleGroupItem | FolderResultItem | FileResultItem | LineResultItem;

export abstract class BaseIssuesView implements vscode.TreeDataProvider<vscode.TreeItem> {
  protected results: IssueResult[] = [];
  protected _viewMode: ViewMode = ViewMode.List;
  protected _groupMode: GroupMode = GroupMode.File;
  protected _lastScanTimestamp: number | null = null;
  protected errorMessage: string | null = null;

  protected _onDidChangeTreeData = new vscode.EventEmitter<vscode.TreeItem | undefined>();
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

  get lastScanTimestamp(): number | null {
    return this._lastScanTimestamp;
  }

  abstract setResults(results: IssueResult[], skipTimestampUpdate?: boolean): void;

  getResults(): IssueResult[] {
    return this.results;
  }

  getResultCount(): number {
    return this.results.length;
  }

  refresh() {
    this._onDidChangeTreeData.fire(undefined);
  }

  setError(message: string): void {
    this.errorMessage = message;
    this.results = [];
    this._onDidChangeTreeData.fire(undefined);
  }

  clearError(): void {
    this.errorMessage = null;
    this._onDidChangeTreeData.fire(undefined);
  }

  protected groupByRule(): Map<string, IssueResult[]> {
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

  protected groupByFile(results: IssueResult[]): Map<string, IssueResult[]> {
    const grouped = new Map<string, IssueResult[]>();
    for (const result of results) {
      const filePath = result.uri.fsPath;
      if (!grouped.has(filePath)) {
        grouped.set(filePath, []);
      }
      grouped.get(filePath)?.push(result);
    }
    return grouped;
  }

  protected buildTreeItems(results: IssueResult[]): IssuesViewItem[] {
    const workspaceRoot = getCurrentWorkspaceFolder()?.uri.fsPath || '';
    const tree = buildFolderTree(results, workspaceRoot);
    const items: IssuesViewItem[] = [];
    for (const [, node] of tree) {
      if (node.type === NodeKind.Folder) {
        items.push(new FolderResultItem(node));
      } else {
        items.push(new FileResultItem(node.path, node.results));
      }
    }
    return items;
  }

  getTreeItem(element: vscode.TreeItem): vscode.TreeItem {
    return element;
  }

  protected getRootChildren(): IssuesViewItem[] {
    if (this._groupMode === GroupMode.Rule) {
      const grouped = this.groupByRule();
      const sortedEntries = Array.from(grouped.entries()).sort((a, b) => a[0].localeCompare(b[0]));
      return sortedEntries.map(([rule, results]) => new RuleGroupItem(rule, results, this._viewMode));
    }

    if (this._viewMode === ViewMode.List) {
      const grouped = this.groupByFile(this.results);
      const sortedEntries = Array.from(grouped.entries()).sort((a, b) => a[0].localeCompare(b[0]));
      return sortedEntries.map(([path, results]) => new FileResultItem(path, results));
    }

    return this.buildTreeItems(this.results);
  }

  protected getRuleGroupChildren(element: RuleGroupItem): IssuesViewItem[] {
    if (element.viewMode === ViewMode.List) {
      const sortedResults = [...element.results].sort((a, b) => {
        const pathCompare = a.uri.fsPath.localeCompare(b.uri.fsPath);
        if (pathCompare !== 0) return pathCompare;
        return a.line - b.line;
      });
      return sortedResults.map((r) => new LineResultItem(r));
    }
    return this.buildTreeItems(element.results);
  }

  protected getFolderChildren(element: FolderResultItem): IssuesViewItem[] {
    const items: IssuesViewItem[] = [];
    for (const [, node] of element.node.children) {
      if (node.type === NodeKind.Folder) {
        items.push(new FolderResultItem(node));
      } else {
        items.push(new FileResultItem(node.path, node.results));
      }
    }
    return items;
  }

  protected getFileChildren(element: FileResultItem): IssuesViewItem[] {
    const sortedResults = [...element.results].sort((a, b) => a.line - b.line);
    return sortedResults.map((r) => new LineResultItem(r));
  }

  getChildren(element?: vscode.TreeItem): Thenable<vscode.TreeItem[]> {
    if (!element) {
      if (this.errorMessage) {
        return Promise.resolve([new ErrorMessageItem(this.errorMessage)]);
      }
      return Promise.resolve(this.getRootChildren());
    }

    if (element instanceof RuleGroupItem) {
      return Promise.resolve(this.getRuleGroupChildren(element));
    }

    if (element instanceof FolderResultItem) {
      return Promise.resolve(this.getFolderChildren(element));
    }

    if (element instanceof FileResultItem) {
      return Promise.resolve(this.getFileChildren(element));
    }

    return Promise.resolve([]);
  }
}
