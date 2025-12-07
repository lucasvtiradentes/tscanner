import { ViewMode } from 'tscanner-common';
import { getCurrentWorkspaceFolder } from '../../common/lib/vscode-utils';
import { type FileNode, type FolderNode, type IssueResult, NodeKind } from '../../common/types';
import { buildFolderTree } from '../components/tree-builder';
import { FolderResultItem } from '../components/tree-items';
import { BaseIssuesView } from './base-issues-view';

export class RegularIssuesView extends BaseIssuesView {
  setResults(results: IssueResult[]): void {
    this.results = results.filter((r) => r.isAi !== true);
    this._onDidChangeTreeData.fire(undefined);
  }

  getAllFolderItems(): FolderResultItem[] {
    if (this._viewMode !== ViewMode.Tree) {
      return [];
    }

    const workspaceRoot = getCurrentWorkspaceFolder()?.uri.fsPath || '';
    const tree = buildFolderTree(this.results, workspaceRoot);
    const folders: FolderResultItem[] = [];

    const collectFolders = (map: Map<string, FolderNode | FileNode>) => {
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
}
