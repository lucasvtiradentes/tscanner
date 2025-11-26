#!/usr/bin/env node

import { spawn } from 'node:child_process';
import { getBinaryPath } from './binary-resolver';

function main(): void {
  try {
    const binaryPath = getBinaryPath();

    const child = spawn(binaryPath, process.argv.slice(2), {
      stdio: 'inherit',
      windowsHide: true,
    });

    child.on('exit', (code, signal) => {
      if (signal) {
        process.kill(process.pid, signal);
      } else {
        process.exit(code || 0);
      }
    });

    process.on('SIGINT', () => {
      child.kill('SIGINT');
      child.kill('SIGTERM');
    });

    process.on('SIGTERM', () => {
      child.kill('SIGTERM');
    });
  } catch (error) {
    const err = error as Error;
    console.error(err.message);
    process.exit(1);
  }
}

main();
