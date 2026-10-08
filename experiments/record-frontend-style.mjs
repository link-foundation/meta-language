#!/usr/bin/env node
// Record compiler-suggested style changes as reproducible shared decorators.
// First generate the frontend rules and save a copy, then run cargo clippy
// --fix on the generated file. This experiment records the resulting diff;
// generation and the stage-parity tests must still verify every adjustment.
import { execFileSync } from 'node:child_process';
import { appendFileSync, readFileSync } from 'node:fs';

const [before, after, output] = process.argv.slice(2);
if (!before || !after || !output) throw new Error('usage: record-frontend-style.mjs BEFORE AFTER DECORATORS');
let diff;
try {
  diff = execFileSync('diff', ['-U', '3', before, after], { encoding: 'utf8' });
} catch (error) {
  if (error.status !== 1) throw error;
  diff = error.stdout;
}
const encode = (text) => encodeURIComponent(text).replace(/[!'()*]/gu, (character) => `%${character.charCodeAt(0).toString(16).toUpperCase()}`);
let order = Math.max(...[...readFileSync(output, 'utf8').matchAll(/\(order (\d+)\)/gu)].map(([, number]) => Number(number))) + 1;
for (const hunk of diff.split(/^@@[^\n]*\n/mu).slice(1)) {
  const lines = hunk.trimEnd().split('\n');
  const previous = lines.filter((line) => !line.startsWith('+')).map((line) => line.slice(1)).join('\n');
  const next = lines.filter((line) => !line.startsWith('-')).map((line) => line.slice(1)).join('\n');
  if (!previous || previous === next) continue;
  appendFileSync(output, `(decorator compiler-style-${order} (level emitter) (order ${order}) (when (format Rust) (scope formatted-module)) (replace source ${encode(previous)} ${encode(next)}))\n`);
  order += 1;
}
