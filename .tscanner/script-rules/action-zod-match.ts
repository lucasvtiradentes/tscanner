#!/usr/bin/env npx tsx

import { type ScriptIssue, addIssue, runScript } from '../../packages/cli/src/types';

function kebabToCamel(str: string): string {
  return str.replace(/-([a-z])/g, (_, c) => c.toUpperCase());
}

function parseActionYmlInputs(content: string): { name: string; line: number }[] {
  const inputs: { name: string; line: number }[] = [];
  const lines = content.split('\n');
  let inInputsSection = false;
  let currentIndent = 0;

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    const trimmed = line.trimStart();

    if (trimmed.startsWith('inputs:')) {
      inInputsSection = true;
      currentIndent = line.length - trimmed.length + 2;
      continue;
    }

    if (inInputsSection) {
      const lineIndent = line.length - trimmed.length;

      if (trimmed.length > 0 && lineIndent < currentIndent && !trimmed.startsWith('#')) {
        break;
      }

      if (lineIndent === currentIndent && trimmed.includes(':')) {
        const inputName = trimmed.split(':')[0].trim();
        if (inputName && !inputName.startsWith('#')) {
          inputs.push({ name: inputName, line: i + 1 });
        }
      }
    }
  }

  return inputs;
}

function parseZodSchemaFields(content: string): string[] {
  const fields: string[] = [];

  const actionYmlSchemaMatch = content.match(
    /const\s+(?:actionYmlInputsSchema|actionInputs)\s*=\s*z\.object\(\{([^}]+)\}\)/s,
  );

  if (!actionYmlSchemaMatch) {
    const simpleSchemaMatch = content.match(/z\.object\(\{([^}]+)\}\)/s);
    if (simpleSchemaMatch) {
      const schemaContent = simpleSchemaMatch[1];
      const regex = /^\s*(\w+):\s*z\./gm;
      for (const match of schemaContent.matchAll(regex)) {
        fields.push(match[1]);
      }
    }
    return fields;
  }

  const schemaContent = actionYmlSchemaMatch[1];
  const regex = /^\s*(\w+):\s*z\./gm;
  for (const match of schemaContent.matchAll(regex)) {
    fields.push(match[1]);
  }

  return fields;
}

runScript((input) => {
  const issues: ScriptIssue[] = [];

  const actionFile = input.files.find((f) => f.path.endsWith('action.yml') || f.path.endsWith('action.yaml'));
  const zodFile = input.files.find((f) => f.path.includes('input-validator') || f.path.includes('inputValidator'));

  if (!actionFile || !zodFile) {
    return issues;
  }

  const actionInputs = parseActionYmlInputs(actionFile.content);
  const zodFields = parseZodSchemaFields(zodFile.content);

  for (const { name: inputName, line } of actionInputs) {
    const camelCaseName = kebabToCamel(inputName);

    if (!zodFields.includes(camelCaseName)) {
      addIssue(issues, {
        file: actionFile.path,
        line,
        message: `Input "${inputName}" is not defined in Zod schema (expected field: "${camelCaseName}")`,
      });
    }
  }

  for (const zodField of zodFields) {
    const kebabName = zodField.replace(/([A-Z])/g, '-$1').toLowerCase();
    const actionInputNames = actionInputs.map((i) => i.name);

    if (!actionInputNames.includes(zodField) && !actionInputNames.includes(kebabName)) {
      const zodLine = zodFile.lines.findIndex((l) => l.includes(`${zodField}:`)) + 1;
      addIssue(issues, {
        file: zodFile.path,
        line: zodLine || 1,
        message: `Zod field "${zodField}" has no matching action.yml input (expected: "${kebabName}")`,
      });
    }
  }

  return issues;
});
