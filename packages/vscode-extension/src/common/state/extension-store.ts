import { DEFAULT_TARGET_BRANCH, ScanMode, type TscannerConfig } from 'tscanner-common';
import type * as vscode from 'vscode';
import { createLogger } from '../lib/logger';
import { ContextKey, WorkspaceStateKey, getWorkspaceState, setContextKey, setWorkspaceState } from './workspace-state';

const storeLogger = createLogger('store');

export enum StoreKey {
  IsSearching = 'isSearching',
  IsAiSearching = 'isAiSearching',
  ScanMode = 'scanMode',
  CompareBranch = 'compareBranch',
  ConfigDir = 'configDir',
  CachedConfig = 'cachedConfig',
}

type ExtensionState = {
  [StoreKey.IsSearching]: boolean;
  [StoreKey.IsAiSearching]: boolean;
  [StoreKey.ScanMode]: ScanMode;
  [StoreKey.CompareBranch]: string;
  [StoreKey.ConfigDir]: string | null;
  [StoreKey.CachedConfig]: TscannerConfig | null;
};

type StateListener<K extends StoreKey> = (value: ExtensionState[K], oldValue: ExtensionState[K]) => void;
type AnyStateListener = StateListener<StoreKey>;

class ExtensionStore {
  private state: ExtensionState = {
    [StoreKey.IsSearching]: false,
    [StoreKey.IsAiSearching]: false,
    [StoreKey.ScanMode]: ScanMode.Codebase,
    [StoreKey.CompareBranch]: DEFAULT_TARGET_BRANCH,
    [StoreKey.ConfigDir]: null,
    [StoreKey.CachedConfig]: null,
  };

  private context: vscode.ExtensionContext | null = null;
  private listeners = new Map<StoreKey, Set<AnyStateListener>>();

  initialize(context: vscode.ExtensionContext): void {
    this.context = context;
    this.state[StoreKey.ScanMode] = getWorkspaceState(context, WorkspaceStateKey.ScanMode);
    this.state[StoreKey.CompareBranch] = getWorkspaceState(context, WorkspaceStateKey.CompareBranch);
    this.state[StoreKey.ConfigDir] = getWorkspaceState(context, WorkspaceStateKey.ConfigDir);
    storeLogger.debug('Initialized with persisted state');
  }

  get<K extends StoreKey>(key: K): ExtensionState[K] {
    return this.state[key];
  }

  set<K extends StoreKey>(key: K, value: ExtensionState[K]): void {
    const oldValue = this.state[key];
    if (oldValue === value) return;

    this.state[key] = value;
    storeLogger.debug(`${key}: ${JSON.stringify(oldValue)} -> ${JSON.stringify(value)}`);

    this.persist(key, value);
    this.notify(key, value, oldValue);
  }

  subscribe<K extends StoreKey>(key: K, listener: StateListener<K>): () => void {
    let keyListeners = this.listeners.get(key);
    if (!keyListeners) {
      keyListeners = new Set();
      this.listeners.set(key, keyListeners);
    }
    keyListeners.add(listener as AnyStateListener);
    return () => {
      this.listeners.get(key)?.delete(listener as AnyStateListener);
    };
  }

  private persist<K extends StoreKey>(key: K, value: ExtensionState[K]): void {
    if (!this.context) return;

    switch (key) {
      case StoreKey.ScanMode:
        setWorkspaceState(this.context, WorkspaceStateKey.ScanMode, value as ScanMode);
        setContextKey(ContextKey.ScanMode, value);
        break;
      case StoreKey.CompareBranch:
        setWorkspaceState(this.context, WorkspaceStateKey.CompareBranch, value as string);
        break;
      case StoreKey.ConfigDir:
        setWorkspaceState(this.context, WorkspaceStateKey.ConfigDir, value as string | null);
        break;
      case StoreKey.IsSearching:
        setContextKey(ContextKey.Searching, value);
        break;
      case StoreKey.IsAiSearching:
        setContextKey(ContextKey.AiSearching, value);
        break;
    }
  }

  private notify<K extends StoreKey>(key: K, value: ExtensionState[K], oldValue: ExtensionState[K]): void {
    const keyListeners = this.listeners.get(key);
    if (!keyListeners) return;

    for (const listener of keyListeners) {
      try {
        listener(value, oldValue);
      } catch (err) {
        storeLogger.error(`Listener error for ${key}: ${err}`);
      }
    }
  }
}

export const extensionStore = new ExtensionStore();
