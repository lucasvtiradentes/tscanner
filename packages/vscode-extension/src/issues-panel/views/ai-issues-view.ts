import * as vscode from 'vscode';
import type { IssueResult } from '../../common/types';
import type { AiProgressParams, AiRuleStatus } from '../../lsp/requests/types';
import { BaseIssuesView } from './base-issues-view';

type RuleProgressState = {
  ruleName: string;
  ruleIndex: number;
  totalRules: number;
  status: AiRuleStatus;
};

class AiProgressItem extends vscode.TreeItem {
  constructor(state: RuleProgressState) {
    const icon = getStatusIcon(state.status);
    const suffix = getStatusSuffix(state.status);
    super(`${icon} ${state.ruleName}${suffix}`, vscode.TreeItemCollapsibleState.None);
    this.contextValue = 'aiProgressItem';
  }
}

function getStatusIcon(status: AiRuleStatus): string {
  if ('pending' in status) return '○';
  if ('running' in status) return '⧗';
  if ('completed' in status) return '✓';
  if ('failed' in status) return '✗';
  return '?';
}

function getStatusSuffix(status: AiRuleStatus): string {
  if ('completed' in status && status.completed.issues_found > 0) {
    const count = status.completed.issues_found;
    return ` (${count} ${count === 1 ? 'issue' : 'issues'})`;
  }
  return '';
}

export class AiIssuesView extends BaseIssuesView {
  private _lastScanTimestamp: number | null = null;
  private _progressStates: Map<number, RuleProgressState> = new Map();
  private _isShowingProgress = false;

  get lastScanTimestamp(): number | null {
    return this._lastScanTimestamp;
  }

  setResults(results: IssueResult[], skipTimestampUpdate?: boolean): void {
    this.results = results.filter((r) => r.isAi === true);
    if (!skipTimestampUpdate) {
      this._lastScanTimestamp = Date.now();
    }
    this._isShowingProgress = false;
    this._progressStates.clear();
    this._onDidChangeTreeData.fire(undefined);
  }

  updateProgress(params: AiProgressParams): void {
    this._isShowingProgress = true;
    this._progressStates.set(params.rule_index, {
      ruleName: params.rule_name,
      ruleIndex: params.rule_index,
      totalRules: params.total_rules,
      status: params.status,
    });
    this._onDidChangeTreeData.fire(undefined);
  }

  clearProgress(): void {
    this._isShowingProgress = false;
    this._progressStates.clear();
    this._onDidChangeTreeData.fire(undefined);
  }

  override getChildren(element?: vscode.TreeItem): Thenable<vscode.TreeItem[]> {
    if (!element && this._isShowingProgress) {
      const progressItems: AiProgressItem[] = [];
      const sortedKeys = [...this._progressStates.keys()].sort((a, b) => a - b);
      for (const key of sortedKeys) {
        const state = this._progressStates.get(key);
        if (state) {
          progressItems.push(new AiProgressItem(state));
        }
      }
      return Promise.resolve(progressItems);
    }
    return super.getChildren(element);
  }
}
