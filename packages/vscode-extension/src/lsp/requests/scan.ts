import { RequestType } from 'vscode-languageclient/node';
import type { ScanResult } from '../../common/types';
import type { ScanParams } from './types';

export const ScanRequestType = new RequestType<ScanParams, ScanResult, void>('tscanner/scan');
