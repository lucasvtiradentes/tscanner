import { LspMethod } from 'tscanner-common';
import { RequestType } from 'vscode-languageclient/node';
import type { ValidateConfigParams, ValidateConfigResult } from './types';

export const ValidateConfigRequestType = new RequestType<ValidateConfigParams, ValidateConfigResult, void>(
  LspMethod.ValidateConfig,
);
