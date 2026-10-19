#!/usr/bin/env node
// Translate the pure TypeScript erasure decisions through the native frontend.
import { spawnSync } from 'node:child_process';
import { readFileSync, writeFileSync } from 'node:fs';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { DecoratorSet } from '../src/decorators.js';

const source = new URL('../src/typescript-erasure-rules.js', import.meta.url);
const target = new URL('../../rust/src/self_translation/typescript_erasure_rules.rs', import.meta.url);
const data = readFileSync(new URL('../src/data/typescript-erasure.lino', import.meta.url), 'utf8');
const dataTarget = new URL('../../rust/src/data/typescript-erasure.lino', import.meta.url);
const typing = readFileSync(new URL('../src/data/typescript-portable-typing.lino', import.meta.url), 'utf8');
const typingTarget = new URL('../../rust/src/data/typescript-portable-typing.lino', import.meta.url);
const decorators = DecoratorSet.fromLino(readFileSync(new URL('../../parity/self-translation/typescript-erasure-rules-decorators.lino', import.meta.url), 'utf8'));
const emitted = emitRust(checkProgram(parseJavaScript(readFileSync(source, 'utf8'))));
const decorated = decorators.decorate('emitter', { format: 'Rust', source: emitted.definitions.join('\n\n') }).source;
// Source CSTs retain Unicode scalars. The reviewed documentation adapter replaces
// an ASCII delimiter identically over UTF-8 and UTF-16 without an auxiliary runtime.
if (decorated.includes('crate::ml_string')) throw new Error('TypeScript source rule style left an unbound string runtime');
const formatted = spawnSync('rustfmt', ['--edition', '2024', '--emit', 'stdout'], { input: decorated, encoding: 'utf8', maxBuffer: 1024 * 1024 });
if (formatted.error || formatted.status !== 0) throw formatted.error ?? new Error(formatted.stderr);
const output = '// Generated from js/src/typescript-erasure-rules.js by\n'
  + '// js/scripts/generate-typescript-erasure-rules.mjs. Do not edit by hand.\n' + formatted.stdout;
if (process.argv.includes('--check')) {
  if (readFileSync(target, 'utf8') !== output) throw new Error('TypeScript erasure rules are stale');
  if (readFileSync(dataTarget, 'utf8') !== data) throw new Error('shared TypeScript erasure data is stale');
  if (readFileSync(typingTarget, 'utf8') !== typing) throw new Error('shared TypeScript typing data is stale');
} else {
  writeFileSync(target, output);
  writeFileSync(dataTarget, data);
  writeFileSync(typingTarget, typing);
}
