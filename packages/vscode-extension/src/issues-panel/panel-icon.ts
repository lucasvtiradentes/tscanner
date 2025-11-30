import type * as vscode from 'vscode';
import { formatIssueCount } from '../common/lib/vscode-utils';
import type { IssuesPanelContent } from './panel-content';

export class IssuesPanelIcon {
  constructor(
    private readonly treeView: vscode.TreeView<any>,
    private readonly panelContent: IssuesPanelContent,
  ) {}

  update(): void {
    const count = this.panelContent.getResultCount();
    this.treeView.badge = count > 0 ? { value: count, tooltip: formatIssueCount(count) } : undefined;
  }
}
