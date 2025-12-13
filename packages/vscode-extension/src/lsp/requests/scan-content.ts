import { type ContentScanResult, LspMethod } from 'tscanner-common';
import { RequestType } from 'vscode-languageclient/node';
import type { ScanContentParams } from './types';

export const ScanContentRequestType = new RequestType<ScanContentParams, ContentScanResult, void>(
  LspMethod.ScanContent,
);
