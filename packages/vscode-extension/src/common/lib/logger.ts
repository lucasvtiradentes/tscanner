import { appendFileSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { LOG_TIMEZONE_OFFSET_HOURS } from 'tscanner-common';
import { getLogFilename } from '../constants';

export const LOG_FILE_PATH = join(tmpdir(), getLogFilename());

let logsEnabled = false;

export function initializeLogger(enabled: boolean) {
  logsEnabled = enabled;
}

function formatTimestamp(): string {
  const now = new Date();
  const offsetMs = LOG_TIMEZONE_OFFSET_HOURS * 60 * 60 * 1000;
  const localTime = new Date(now.getTime() + offsetMs);

  const year = localTime.getUTCFullYear();
  const month = String(localTime.getUTCMonth() + 1).padStart(2, '0');
  const day = String(localTime.getUTCDate()).padStart(2, '0');
  const hours = String(localTime.getUTCHours()).padStart(2, '0');
  const minutes = String(localTime.getUTCMinutes()).padStart(2, '0');
  const seconds = String(localTime.getUTCSeconds()).padStart(2, '0');
  const ms = String(localTime.getUTCMilliseconds()).padStart(3, '0');

  const sign = LOG_TIMEZONE_OFFSET_HOURS >= 0 ? '+' : '';
  const offsetStr = `${sign}${String(LOG_TIMEZONE_OFFSET_HOURS).padStart(2, '0')}:00`;

  return `${year}-${month}-${day}T${hours}:${minutes}:${seconds}.${ms}${offsetStr}`;
}

class Logger {
  private context: string;

  constructor(context: string) {
    this.context = context;
  }

  private write(level: string, message: string) {
    if (!logsEnabled) return;

    const timestamp = formatTimestamp();
    const logMessage = `[${timestamp}] [${this.context}] [${level}] ${message}`;

    appendFileSync(LOG_FILE_PATH, `${logMessage}\n`);
  }

  info(message: string) {
    this.write('INFO ', message);
  }

  error(message: string) {
    this.write('ERROR', message);
  }

  warn(message: string) {
    this.write('WARN ', message);
  }

  debug(message: string) {
    this.write('DEBUG', message);
  }

  clear() {
    writeFileSync(LOG_FILE_PATH, '');
  }
}

export const logger = new Logger('vscode_extension');
