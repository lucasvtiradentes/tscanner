#!/usr/bin/env node
import { existsSync } from 'fs';
import { dirname, join } from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const postinstallPath = join(__dirname, '..', 'dist', 'scripts', 'postinstall.js');

if (existsSync(postinstallPath)) {
  await import(postinstallPath);
}
