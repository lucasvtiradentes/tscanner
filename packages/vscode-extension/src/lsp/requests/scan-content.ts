import { RequestType } from 'vscode-languageclient/node';
import type { ContentScanResult } from '../../common/types';
import type { ScanContentParams } from './types';

export const ScanContentRequestType = new RequestType<ScanContentParams, ContentScanResult, void>(
  'tscanner/scanContent',
);
