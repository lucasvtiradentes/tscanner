#!/usr/bin/env npx tsx

import { stdin } from 'node:process';

type ScriptFile = {
  path: string;
  content: string;
  lines: string[];
};

type ScriptInput = {
  files: ScriptFile[];
  options?: Record<string, unknown>;
  workspaceRoot: string;
};

type ScriptIssue = {
  file: string;
  line: number;
  column?: number;
  message: string;
};

type RustConfigFields = {
  tscanner_config: string[];
  code_editor_config: string[];
  cli_config: string[];
  files_config: string[];
  builtin_rule_config: string[];
  custom_rule_base: string[];
  regex_rule_config: string[];
  script_rule_config: string[];
  ai_rule_config: string[];
  script_mode: string[];
  custom_rule_types: string[];
};

function extractZodObjectFields(content: string, schemaName: string): string[] {
  const regex = new RegExp(`(?:export )?const ${schemaName}\\s*=\\s*z\\s*\\.object\\(\\{([\\s\\S]*?)\\}\\)`, 'm');
  const match = content.match(regex);
  if (!match) return [];

  const fieldsStr = match[1];
  const fieldMatches = fieldsStr.matchAll(/^\s*(\w+):/gm);
  return [...fieldMatches].map((m) => m[1]);
}

function extractZodEnumValues(content: string, schemaName: string): string[] {
  const regex = new RegExp(`export const ${schemaName}\\s*=\\s*z\\.enum\\(\\[([^\\]]+)\\]`);
  const match = content.match(regex);
  if (!match) return [];

  const valuesStr = match[1];
  const valueMatches = valuesStr.matchAll(/['"]([^'"]+)['"]/g);
  return [...valueMatches].map((m) => m[1]);
}

function findLineNumber(content: string, schemaName: string): number {
  const lines = content.split('\n');
  for (let i = 0; i < lines.length; i++) {
    if (lines[i].includes(`const ${schemaName}`)) {
      return i + 1;
    }
  }
  return 1;
}

function compareFields(
  rustFields: string[],
  tsFields: string[],
  schemaName: string,
  file: string,
  line: number,
  issues: ScriptIssue[],
): void {
  const rustSet = new Set(rustFields);
  const tsSet = new Set(tsFields);

  for (const field of rustFields) {
    if (!tsSet.has(field)) {
      issues.push({
        file,
        line,
        message: `${schemaName}: field "${field}" exists in Rust but missing in TypeScript schema`,
      });
    }
  }

  for (const field of tsFields) {
    if (!rustSet.has(field)) {
      issues.push({
        file,
        line,
        message: `${schemaName}: field "${field}" exists in TypeScript but missing in Rust config`,
      });
    }
  }
}

async function main() {
  let data = '';
  for await (const chunk of stdin) {
    data += chunk;
  }

  const input: ScriptInput = JSON.parse(data);
  const issues: ScriptIssue[] = [];

  const rustConfigFile = input.files.find((f) => f.path.endsWith('rust_config_fields.json'));
  const schemasFile = input.files.find((f) => f.path.endsWith('schemas.ts'));

  if (!rustConfigFile || !schemasFile) {
    console.log(JSON.stringify({ issues }));
    return;
  }

  let rustConfig: RustConfigFields;
  try {
    rustConfig = JSON.parse(rustConfigFile.content);
  } catch {
    issues.push({
      file: rustConfigFile.path,
      line: 1,
      message: 'Failed to parse rust_config_fields.json',
    });
    console.log(JSON.stringify({ issues }));
    return;
  }

  const tsContent = schemasFile.content;

  const tscannerConfigFields = extractZodObjectFields(tsContent, 'tscannerConfigSchema');
  compareFields(
    rustConfig.tscanner_config,
    tscannerConfigFields,
    'tscannerConfigSchema',
    schemasFile.path,
    findLineNumber(tsContent, 'tscannerConfigSchema'),
    issues,
  );

  const codeEditorFields = extractZodObjectFields(tsContent, 'codeEditorConfigSchema');
  compareFields(
    rustConfig.code_editor_config,
    codeEditorFields,
    'codeEditorConfigSchema',
    schemasFile.path,
    findLineNumber(tsContent, 'codeEditorConfigSchema'),
    issues,
  );

  const cliConfigFields = extractZodObjectFields(tsContent, 'cliConfigSchema');
  compareFields(
    rustConfig.cli_config,
    cliConfigFields,
    'cliConfigSchema',
    schemasFile.path,
    findLineNumber(tsContent, 'cliConfigSchema'),
    issues,
  );

  const filesConfigFields = extractZodObjectFields(tsContent, 'filesConfigSchema');
  compareFields(
    rustConfig.files_config,
    filesConfigFields,
    'filesConfigSchema',
    schemasFile.path,
    findLineNumber(tsContent, 'filesConfigSchema'),
    issues,
  );

  const builtinRuleFields = extractZodObjectFields(tsContent, 'builtinRuleConfigSchema');
  compareFields(
    rustConfig.builtin_rule_config,
    builtinRuleFields,
    'builtinRuleConfigSchema',
    schemasFile.path,
    findLineNumber(tsContent, 'builtinRuleConfigSchema'),
    issues,
  );

  const customRuleBaseFields = extractZodObjectFields(tsContent, 'customRuleBaseSchema');
  compareFields(
    rustConfig.custom_rule_base,
    customRuleBaseFields,
    'customRuleBaseSchema',
    schemasFile.path,
    findLineNumber(tsContent, 'customRuleBaseSchema'),
    issues,
  );

  const regexRuleOwnFields = extractZodObjectFields(tsContent, 'regexRuleConfigSchema');
  compareFields(
    rustConfig.regex_rule_config,
    regexRuleOwnFields,
    'regexRuleConfigSchema',
    schemasFile.path,
    findLineNumber(tsContent, 'regexRuleConfigSchema'),
    issues,
  );

  const scriptRuleOwnFields = extractZodObjectFields(tsContent, 'scriptRuleConfigSchema');
  compareFields(
    rustConfig.script_rule_config,
    scriptRuleOwnFields,
    'scriptRuleConfigSchema',
    schemasFile.path,
    findLineNumber(tsContent, 'scriptRuleConfigSchema'),
    issues,
  );

  const aiRuleOwnFields = extractZodObjectFields(tsContent, 'aiRuleConfigSchema');
  compareFields(
    rustConfig.ai_rule_config,
    aiRuleOwnFields,
    'aiRuleConfigSchema',
    schemasFile.path,
    findLineNumber(tsContent, 'aiRuleConfigSchema'),
    issues,
  );

  const scriptModeValues = extractZodEnumValues(tsContent, 'scriptModeSchema');
  compareFields(
    rustConfig.script_mode,
    scriptModeValues,
    'scriptModeSchema',
    schemasFile.path,
    findLineNumber(tsContent, 'scriptModeSchema'),
    issues,
  );

  console.log(JSON.stringify({ issues }));
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
