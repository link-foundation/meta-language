#!/usr/bin/env node
// Generate the merge decisions through meta-language's JavaScript frontend.
import { readFileSync, writeFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { DecoratorSet } from '../src/decorators.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { parseJavaScript } from '../src/translation/javascript.js';

const source = new URL('../src/grammar-merge-decisions.js', import.meta.url);
const target = new URL('../../rust/src/grammar/merge/decisions.rs', import.meta.url);
const rules = new URL('../../parity/self-translation/grammar-merge-decisions-decorators.lino', import.meta.url);
const decorators = DecoratorSet.fromLino(readFileSync(rules, 'utf8'));
const translated = emitRust(checkProgram(parseJavaScript(readFileSync(source, 'utf8'))));
const decorated = decorators.decorate('emitter', { format: 'Rust', source: translated.text }).source;
const formatted = spawnSync('rustfmt', ['--edition', '2024', '--emit', 'stdout'], {
  input: decorated, encoding: 'utf8', maxBuffer: 1024 * 1024,
});
if (formatted.error || formatted.status !== 0) throw formatted.error ?? new Error(formatted.stderr);
const text = '// Generated from js/src/grammar-merge-decisions.js by\n'
  + '// js/scripts/generate-grammar-merge-decisions.mjs. Do not edit by hand.\n' + formatted.stdout;
if (process.argv.includes('--check')) {
  if (readFileSync(target, 'utf8') !== text) throw new Error('grammar merge decisions are stale');
} else writeFileSync(target, text);
