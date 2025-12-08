import type { FileResult } from 'tscanner-common';
import { RequestType } from 'vscode-languageclient/node';
import type { ScanFileParams } from './types';

export const ScanFileRequestType = new RequestType<ScanFileParams, FileResult, void>('tscanner/scanFile');
