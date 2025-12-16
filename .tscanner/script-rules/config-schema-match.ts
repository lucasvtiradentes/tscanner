#!/usr/bin/env npx tsx

import { stdin } from 'node:process';
import type { ScriptInput, ScriptIssue } from '../../shared/tscanner-common/src';

type JsonSchema = {
  properties?: Record<string, unknown>;
  definitions?: Record<string, { properties?: Record<string, unknown> }>;
};

function extractSchemaFields(schema: JsonSchema, definitionName: string): string[] {
  const definition = schema.definitions?.[definitionName];
  if (!definition?.properties) return [];
  return Object.keys(definition.properties);
}

function extractTopLevelFields(schema: JsonSchema): string[] {
  if (!schema.properties) return [];
  return Object.keys(schema.properties);
}

function extractZodObjectFields(content: string, schemaName: string): string[] {
  const baseSchemaMatch = content.match(
    new RegExp(
      `(?:export )?const ${schemaName}\\s*=\\s*baseRuleConfigSchema\\s*\\.extend\\(\\{([\\s\\S]*?)\\}\\)`,
      'm',
    ),
  );
  if (baseSchemaMatch) {
    const fieldsStr = baseSchemaMatch[1];
    const fieldMatches = fieldsStr.matchAll(/^\s*(\w+):/gm);
    return [...fieldMatches].map((m) => m[1]);
  }

  const assignMatch = content.match(
    new RegExp(`(?:export )?const ${schemaName}\\s*=\\s*baseRuleConfigSchema\\s*;`, 'm'),
  );
  if (assignMatch) {
    return [];
  }

  const regex = new RegExp(`(?:export )?const ${schemaName}\\s*=\\s*z\\s*\\.object\\(\\{([\\s\\S]*?)\\}\\)`, 'm');
  const match = content.match(regex);
  if (!match) return [];

  const fieldsStr = match[1];
  const fieldMatches = fieldsStr.matchAll(/^\s*(\w+):/gm);
  return [...fieldMatches].map((m) => m[1]);
}

function extractBaseRuleFields(content: string): string[] {
  const regex = /const baseRuleConfigSchema\s*=\s*z\s*\.object\(\{([\s\S]*?)\}\)/m;
  const match = content.match(regex);
  if (!match) return [];

  const fieldsStr = match[1];
  const fieldMatches = fieldsStr.matchAll(/^\s*(\w+):/gm);
  return [...fieldMatches].map((m) => m[1]);
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
        message: `${schemaName}: field "${field}" exists in Rust schema but missing in TypeScript`,
      });
    }
  }

  for (const field of tsFields) {
    if (!rustSet.has(field)) {
      issues.push({
        file,
        line,
        message: `${schemaName}: field "${field}" exists in TypeScript but missing in Rust schema`,
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

  const jsonSchemaFile = input.files.find((f) => f.path.endsWith('schema.json'));
  const schemasFile = input.files.find((f) => f.path.endsWith('schemas.ts'));

  if (!jsonSchemaFile || !schemasFile) {
    console.log(JSON.stringify({ issues }));
    return;
  }

  let jsonSchema: JsonSchema;
  try {
    jsonSchema = JSON.parse(jsonSchemaFile.content);
  } catch {
    issues.push({
      file: jsonSchemaFile.path,
      line: 1,
      message: 'Failed to parse schema.json',
    });
    console.log(JSON.stringify({ issues }));
    return;
  }

  const tsContent = schemasFile.content;
  const baseRuleFields = extractBaseRuleFields(tsContent);

  const tscannerConfigFields = extractZodObjectFields(tsContent, 'tscannerConfigSchema');
  compareFields(
    extractTopLevelFields(jsonSchema).filter((f) => f !== '$schema'),
    tscannerConfigFields,
    'tscannerConfigSchema',
    schemasFile.path,
    findLineNumber(tsContent, 'tscannerConfigSchema'),
    issues,
  );

  const aiConfigFields = extractZodObjectFields(tsContent, 'aiConfigSchema');
  compareFields(
    extractSchemaFields(jsonSchema, 'AiConfig'),
    aiConfigFields,
    'aiConfigSchema',
    schemasFile.path,
    findLineNumber(tsContent, 'aiConfigSchema'),
    issues,
  );

  const codeEditorFields = extractZodObjectFields(tsContent, 'codeEditorConfigSchema');
  compareFields(
    extractSchemaFields(jsonSchema, 'CodeEditorConfig'),
    codeEditorFields,
    'codeEditorConfigSchema',
    schemasFile.path,
    findLineNumber(tsContent, 'codeEditorConfigSchema'),
    issues,
  );

  const cliConfigFields = extractZodObjectFields(tsContent, 'cliConfigSchema');
  compareFields(
    extractSchemaFields(jsonSchema, 'CliConfig'),
    cliConfigFields,
    'cliConfigSchema',
    schemasFile.path,
    findLineNumber(tsContent, 'cliConfigSchema'),
    issues,
  );

  const filesConfigFields = extractZodObjectFields(tsContent, 'filesConfigSchema');
  compareFields(
    extractSchemaFields(jsonSchema, 'FilesConfig'),
    filesConfigFields,
    'filesConfigSchema',
    schemasFile.path,
    findLineNumber(tsContent, 'filesConfigSchema'),
    issues,
  );

  const rulesConfigFields = extractZodObjectFields(tsContent, 'rulesConfigSchema');
  compareFields(
    extractSchemaFields(jsonSchema, 'RulesConfig'),
    rulesConfigFields,
    'rulesConfigSchema',
    schemasFile.path,
    findLineNumber(tsContent, 'rulesConfigSchema'),
    issues,
  );

  const builtinRuleFields = extractZodObjectFields(tsContent, 'builtinRuleConfigSchema');
  const allBuiltinFields = [...baseRuleFields, ...builtinRuleFields];
  compareFields(
    extractSchemaFields(jsonSchema, 'BuiltinRuleConfig'),
    allBuiltinFields,
    'builtinRuleConfigSchema',
    schemasFile.path,
    findLineNumber(tsContent, 'builtinRuleConfigSchema'),
    issues,
  );

  const regexRuleOwnFields = extractZodObjectFields(tsContent, 'regexRuleConfigSchema');
  const allRegexFields = [...baseRuleFields, ...regexRuleOwnFields];
  compareFields(
    extractSchemaFields(jsonSchema, 'RegexRuleConfig'),
    allRegexFields,
    'regexRuleConfigSchema',
    schemasFile.path,
    findLineNumber(tsContent, 'regexRuleConfigSchema'),
    issues,
  );

  const scriptRuleOwnFields = extractZodObjectFields(tsContent, 'scriptRuleConfigSchema');
  const allScriptFields = [...baseRuleFields, ...scriptRuleOwnFields];
  compareFields(
    extractSchemaFields(jsonSchema, 'ScriptRuleConfig'),
    allScriptFields,
    'scriptRuleConfigSchema',
    schemasFile.path,
    findLineNumber(tsContent, 'scriptRuleConfigSchema'),
    issues,
  );

  const aiRuleOwnFields = extractZodObjectFields(tsContent, 'aiRuleConfigSchema');
  const allAiRuleFields = [...baseRuleFields, ...aiRuleOwnFields];
  compareFields(
    extractSchemaFields(jsonSchema, 'AiRuleConfig'),
    allAiRuleFields,
    'aiRuleConfigSchema',
    schemasFile.path,
    findLineNumber(tsContent, 'aiRuleConfigSchema'),
    issues,
  );

  console.log(JSON.stringify({ issues }));
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
