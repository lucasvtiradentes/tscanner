import { LspMethod } from 'tscanner-common';
import { RequestType } from 'vscode-languageclient/node';
import type { FormatPrettyResult, FormatResultsParams } from './types';

export const FormatResultsRequestType = new RequestType<FormatResultsParams, FormatPrettyResult, void>(
  LspMethod.FormatResults,
);
