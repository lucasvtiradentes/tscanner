import type { RuleMetadata } from 'tscanner-common';
import { RequestType0 } from 'vscode-languageclient/node';

export const GetRulesMetadataRequestType = new RequestType0<RuleMetadata[], void>('tscanner/getRulesMetadata');
