import { RequestType } from 'vscode-languageclient/node';
import type { FileResult } from '../../common/types';
import type { ScanFileParams } from './types';

export const ScanFileRequestType = new RequestType<ScanFileParams, FileResult, void>('tscanner/scanFile');
