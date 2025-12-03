import { RequestType0 } from 'vscode-languageclient/node';
import type { RuleMetadata } from './types';

export const GetRulesMetadataRequestType = new RequestType0<RuleMetadata[], void>('tscanner/getRulesMetadata');
