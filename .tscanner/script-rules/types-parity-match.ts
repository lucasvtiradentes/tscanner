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

type FieldInfo = {
  name: string;
  optional: boolean;
};

type TypeDef = {
  name: string;
  kind: 'struct' | 'enum';
  fields: FieldInfo[];
  line: number;
};

const FILE_PAIRS: [string, string][] = [
  ['enums.ts', 'enums.rs'],
  ['issue.ts', 'issue.rs'],
  ['results.ts', 'results.rs'],
  ['metadata.ts', 'metadata.rs'],
  ['params.ts', 'params.rs'],
  ['display.ts', 'display.rs'],
  ['config.ts', 'config.rs'],
  ['cli-output.ts', 'cli_output.rs'],
];

const EXCLUDED_TYPES = new Set(['RuleOption']);

const FIELD_MAPPINGS: Record<string, Record<string, string>> = {
  TscannerConfig: { $schema: 'schema' },
};

function toSnakeCase(str: string): string {
  return str.replace(/([a-z])([A-Z])/g, '$1_$2').toLowerCase();
}

function toCamelCase(str: string): string {
  return str.replace(/_([a-z])/g, (_, c) => c.toUpperCase());
}

function extractTsEnums(content: string): TypeDef[] {
  const enums: TypeDef[] = [];
  const enumRegex = /export enum (\w+)\s*\{([^}]+)\}/g;

  for (const match of content.matchAll(enumRegex)) {
    const name = match[1];
    const body = match[2];
    const variants = body
      .split(',')
      .map((v) => v.trim())
      .filter((v) => v && !v.startsWith('//'))
      .map((v) => {
        const [varName] = v.split('=').map((s) => s.trim());
        return { name: varName, optional: false };
      });

    const line = content.substring(0, match.index!).split('\n').length;
    enums.push({ name, kind: 'enum', fields: variants, line });
  }

  return enums;
}

function toPascalCase(str: string): string {
  return str.charAt(0).toUpperCase() + str.slice(1);
}

function extractTsZodTypes(content: string): TypeDef[] {
  const types: TypeDef[] = [];
  const schemaRegex = /(?:const|export const) (\w+Schema)\s*=\s*z\.object\(\{([^}]+(?:\{[^}]*\}[^}]*)*)\}\)/g;

  for (const match of content.matchAll(schemaRegex)) {
    const schemaName = match[1];
    const name = toPascalCase(schemaName.replace(/Schema$/, ''));
    const body = match[2];

    const fields: FieldInfo[] = [];
    const fieldRegex = /^\s*([\w$]+):\s*(.+?)(?:,\s*$|$)/gm;

    for (const fieldMatch of body.matchAll(fieldRegex)) {
      const fieldName = fieldMatch[1];
      const fieldDef = fieldMatch[2];
      const optional = fieldDef.includes('.optional()') || fieldDef.includes('.default(');
      fields.push({ name: fieldName, optional });
    }

    const line = content.substring(0, match.index!).split('\n').length;
    types.push({ name, kind: 'struct', fields, line });
  }

  return types;
}

function extractRustEnums(content: string): TypeDef[] {
  const enums: TypeDef[] = [];
  const enumRegex = /pub enum (\w+)\s*\{([^}]+)\}/g;

  for (const match of content.matchAll(enumRegex)) {
    const name = match[1];
    const body = match[2];
    const variants: FieldInfo[] = [];

    const cleanBody = body.replace(/#\[[^\]]*\]/g, '').replace(/\/\/[^\n]*/g, '');

    const variantMatches = cleanBody.matchAll(/\b([A-Z][a-zA-Z0-9]*)\b/g);
    for (const vm of variantMatches) {
      const varName = vm[1];
      if (!variants.find((v) => v.name === varName)) {
        variants.push({ name: varName, optional: false });
      }
    }

    const lineNum = content.substring(0, match.index!).split('\n').length;
    enums.push({ name, kind: 'enum', fields: variants, line: lineNum });
  }

  return enums;
}

function extractRustStructs(content: string): TypeDef[] {
  const structs: TypeDef[] = [];
  const structRegex = /pub struct (\w+)\s*\{([^}]+)\}/g;

  for (const match of content.matchAll(structRegex)) {
    const name = match[1];
    const body = match[2];

    const fields: FieldInfo[] = [];
    const lines = body.split('\n');

    for (const line of lines) {
      const fieldMatch = line.match(/^\s*pub (\w+):\s*(.+),?\s*$/);
      if (fieldMatch) {
        const fieldName = fieldMatch[1];
        const fieldType = fieldMatch[2];
        const optional = fieldType.includes('Option<');
        fields.push({ name: fieldName, optional });
      }
    }

    const lineNum = content.substring(0, match.index!).split('\n').length;
    structs.push({ name, kind: 'struct', fields, line: lineNum });
  }

  return structs;
}

function findMatchingType(tsDef: TypeDef, rustDefs: TypeDef[]): TypeDef | undefined {
  return rustDefs.find((r) => r.name === tsDef.name && r.kind === tsDef.kind);
}

function compareTypes(tsFile: ScriptFile, rustFile: ScriptFile, issues: ScriptIssue[]): void {
  const tsEnums = extractTsEnums(tsFile.content);
  const tsTypes = extractTsZodTypes(tsFile.content);
  const rustEnums = extractRustEnums(rustFile.content);
  const rustStructs = extractRustStructs(rustFile.content);

  for (const tsEnum of tsEnums) {
    const rustEnum = findMatchingType(tsEnum, rustEnums);
    if (!rustEnum) {
      issues.push({
        file: tsFile.path,
        line: tsEnum.line,
        message: `Enum "${tsEnum.name}" not found in Rust (${rustFile.path})`,
      });
      continue;
    }

    const tsVariants = new Set(tsEnum.fields.map((f) => f.name));
    const rustVariants = new Set(rustEnum.fields.map((f) => f.name));

    for (const variant of tsVariants) {
      if (!rustVariants.has(variant)) {
        issues.push({
          file: tsFile.path,
          line: tsEnum.line,
          message: `Enum "${tsEnum.name}": variant "${variant}" missing in Rust`,
        });
      }
    }

    for (const variant of rustVariants) {
      if (!tsVariants.has(variant)) {
        issues.push({
          file: tsFile.path,
          line: tsEnum.line,
          message: `Enum "${tsEnum.name}": variant "${variant}" exists in Rust but missing in TypeScript`,
        });
      }
    }
  }

  for (const tsType of tsTypes) {
    if (EXCLUDED_TYPES.has(tsType.name)) {
      continue;
    }

    const rustStruct = rustStructs.find((r) => r.name === tsType.name);
    if (!rustStruct) {
      continue;
    }

    const fieldMap = FIELD_MAPPINGS[tsType.name] || {};
    const tsFields = new Map(
      tsType.fields.map((f) => {
        const mappedName = fieldMap[f.name] || toSnakeCase(f.name);
        return [mappedName, f];
      }),
    );
    const rustFields = new Map(rustStruct.fields.map((f) => [f.name, f]));

    for (const [snakeName, tsField] of tsFields) {
      if (!rustFields.has(snakeName)) {
        issues.push({
          file: tsFile.path,
          line: tsType.line,
          message: `Type "${tsType.name}": field "${tsField.name}" (${snakeName}) missing in Rust`,
        });
      }
    }

    for (const [rustName] of rustFields) {
      const camelName = toCamelCase(rustName);
      if (!tsFields.has(rustName)) {
        issues.push({
          file: tsFile.path,
          line: tsType.line,
          message: `Type "${tsType.name}": field "${rustName}" (${camelName}) exists in Rust but missing in TypeScript`,
        });
      }
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

  const tsFiles = input.files.filter((f) => f.path.includes('tscanner-common/src/types/'));
  const rustFiles = input.files.filter((f) => f.path.includes('tscanner_types/src/'));

  for (const [tsFileName, rustFileName] of FILE_PAIRS) {
    const tsFile = tsFiles.find((f) => f.path.endsWith(tsFileName));
    const rustFile = rustFiles.find((f) => f.path.endsWith(rustFileName));

    if (!tsFile || !rustFile) {
      continue;
    }

    compareTypes(tsFile, rustFile, issues);
  }

  console.log(JSON.stringify({ issues }));
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
