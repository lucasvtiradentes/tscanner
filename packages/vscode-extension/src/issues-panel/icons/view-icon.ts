import { VSCODE_EXTENSION } from 'tscanner-common';
import type * as vscode from 'vscode';
import { formatIssueCount } from '../../common/lib/vscode-utils';
import type { AiIssuesView } from '../views/ai-issues-view';
import type { BaseIssuesView } from '../views/base-issues-view';

function formatRelativeTime(timestamp: number): string {
  const now = Date.now();
  const diffMs = now - timestamp;
  const diffSeconds = Math.floor(diffMs / 1000);
  const diffMinutes = Math.floor(diffSeconds / 60);
  const diffHours = Math.floor(diffMinutes / 60);
  const diffDays = Math.floor(diffHours / 24);

  if (diffSeconds < 60) return 'just now';
  if (diffMinutes === 1) return '1 min ago';
  if (diffMinutes < 60) return `${diffMinutes} min ago`;
  if (diffHours === 1) return '1 hour ago';
  if (diffHours < 24) return `${diffHours} hours ago`;
  if (diffDays === 1) return '1 day ago';
  return `${diffDays} days ago`;
}

export class IssuesViewIcon {
  private readonly disposable: vscode.Disposable;
  private descriptionInterval: ReturnType<typeof setInterval> | null = null;

  constructor(
    private readonly treeView: vscode.TreeView<vscode.TreeItem>,
    private readonly view: BaseIssuesView,
    private readonly labelPrefix?: string,
  ) {
    this.disposable = this.view.onDidChangeTreeData(() => this.update());
    this.update();

    if (this.isAiView()) {
      this.descriptionInterval = setInterval(
        () => this.updateDescription(),
        VSCODE_EXTENSION.intervals.descriptionUpdateSeconds * 1000,
      );
    }
  }

  private isAiView(): boolean {
    return 'lastScanTimestamp' in this.view;
  }

  private updateDescription(): void {
    if (!this.isAiView()) return;
    const aiView = this.view as AiIssuesView;
    const timestamp = aiView.lastScanTimestamp;
    this.treeView.description = timestamp ? formatRelativeTime(timestamp) : undefined;
  }

  update(): void {
    const count = this.view.getResultCount();
    this.treeView.badge = count > 0 ? { value: count, tooltip: formatIssueCount(count, this.labelPrefix) } : undefined;
    this.updateDescription();
  }

  dispose(): void {
    this.disposable.dispose();
    if (this.descriptionInterval) {
      clearInterval(this.descriptionInterval);
    }
  }
}
