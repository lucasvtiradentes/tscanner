import { DEFAULT_TARGET_BRANCH, GroupMode, ScanMode, ViewMode } from 'tscanner-common';
import * as vscode from 'vscode';
import { z } from 'zod';
import { getContextKey } from '../constants';
import { CONTEXT_PREFIX } from '../scripts-constants';

export enum WorkspaceStateKey {
  ViewMode = 'viewMode',
  GroupMode = 'groupMode',
  ScanMode = 'scanMode',
  CompareBranch = 'compareBranch',
  CachedResults = 'cachedResults',
  ConfigDir = 'customConfigDir',
}

const workspaceStateSchema = z.object({
  [WorkspaceStateKey.ViewMode]: z.enum(ViewMode),
  [WorkspaceStateKey.GroupMode]: z.enum(GroupMode),
  [WorkspaceStateKey.ScanMode]: z.enum(ScanMode),
  [WorkspaceStateKey.CompareBranch]: z.string(),
  [WorkspaceStateKey.CachedResults]: z.array(z.any()),
  [WorkspaceStateKey.ConfigDir]: z.string().nullable(),
});

type WorkspaceStateSchema = z.infer<typeof workspaceStateSchema>;
type WorkspaceStateKeyType = keyof WorkspaceStateSchema;

const defaultValues: WorkspaceStateSchema = {
  [WorkspaceStateKey.ViewMode]: ViewMode.List,
  [WorkspaceStateKey.GroupMode]: GroupMode.File,
  [WorkspaceStateKey.ScanMode]: ScanMode.Codebase,
  [WorkspaceStateKey.CompareBranch]: DEFAULT_TARGET_BRANCH,
  [WorkspaceStateKey.CachedResults]: [],
  [WorkspaceStateKey.ConfigDir]: null,
};

const keyMapping: Record<WorkspaceStateKeyType, string> = Object.fromEntries(
  Object.values(WorkspaceStateKey).map((key) => [key, `${CONTEXT_PREFIX}.${key}`]),
) as Record<WorkspaceStateKeyType, string>;

export enum ContextKey {
  ViewMode = 'tscannerViewMode',
  GroupMode = 'tscannerGroupMode',
  ScanMode = 'tscannerScanMode',
  Searching = 'tscannerSearching',
  AiSearching = 'tscannerAiSearching',
  HasScanned = 'tscannerHasScanned',
  HasAiScanned = 'tscannerHasAiScanned',
}

const contextKeyMapping: Partial<Record<WorkspaceStateKeyType, ContextKey>> = {
  viewMode: ContextKey.ViewMode,
  groupMode: ContextKey.GroupMode,
  scanMode: ContextKey.ScanMode,
};

export function getWorkspaceState<K extends WorkspaceStateKeyType>(
  context: vscode.ExtensionContext,
  key: K,
): WorkspaceStateSchema[K] {
  const storageKey = keyMapping[key];
  const value = context.workspaceState.get(storageKey);
  const defaultValue = defaultValues[key];

  if (value === undefined) {
    return defaultValue;
  }

  const schema = workspaceStateSchema.shape[key];
  const result = schema.safeParse(value);
  const finalValue = (result.success ? result.data : defaultValue) as WorkspaceStateSchema[K];

  return finalValue;
}

export function setWorkspaceState<K extends WorkspaceStateKeyType>(
  context: vscode.ExtensionContext,
  key: K,
  value: WorkspaceStateSchema[K],
): Thenable<void> {
  const storageKey = keyMapping[key];
  return context.workspaceState.update(storageKey, value);
}

export function setContextKey(key: ContextKey, value: unknown): Thenable<unknown> {
  return vscode.commands.executeCommand('setContext', getContextKey(key), value);
}

export function updateState<K extends WorkspaceStateKeyType>(
  context: vscode.ExtensionContext,
  key: K,
  value: WorkspaceStateSchema[K],
): void {
  setWorkspaceState(context, key, value);
  const contextKey = contextKeyMapping[key];
  if (contextKey) {
    setContextKey(contextKey, value);
  }
}
