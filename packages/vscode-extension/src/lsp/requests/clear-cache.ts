import { LspMethod } from 'tscanner-common';
import { RequestType0 } from 'vscode-languageclient/node';
import type { ClearCacheResult } from './types';

export const ClearCacheRequestType = new RequestType0<ClearCacheResult, void>(LspMethod.ClearCache);
