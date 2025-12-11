import { LspMethod, type ScanResult } from 'tscanner-common';
import { RequestType } from 'vscode-languageclient/node';
import type { ScanParams } from './types';

export const ScanRequestType = new RequestType<ScanParams, ScanResult, void>(LspMethod.Scan);
