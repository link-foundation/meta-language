#!/usr/bin/env node
// The `meta-language` command-line tool. `meta-language grammar ...` imports,
// validates, converts, merges, renames, exports and round-trips grammars; see
// runGrammarCommand in grammar-interchange.js, which the Rust binary mirrors.
import { readFileSync } from 'node:fs';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

import { runGrammarCommand } from './grammar-interchange.js';

const USAGE = 'usage: meta-language grammar <command> [options]; run meta-language grammar help\n';

/** Runs the tool over `args` and returns `{ exitCode, stdout, stderr }`. */
export function runCommandLine(args, { readFile = (file) => readFileSync(file, 'utf8') } = {}) {
  const [group, ...rest] = args;
  if (group === 'grammar') return runGrammarCommand(rest, { readFile });
  if (group === '--help' || group === '-h' || group === 'help') return { exitCode: 0, stdout: USAGE, stderr: '' };
  return { exitCode: 2, stdout: '', stderr: group === undefined ? USAGE : `error: unknown command ${group}\n${USAGE}` };
}

if (process.argv[1] && fileURLToPath(import.meta.url) === process.argv[1]) {
  const { exitCode, stdout, stderr } = runCommandLine(process.argv.slice(2));
  process.stdout.write(stdout);
  process.stderr.write(stderr);
  process.exitCode = exitCode;
}
