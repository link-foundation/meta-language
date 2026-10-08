#!/usr/bin/env node
// Translate the JavaScript frontend decisions into the Rust module that the
// lexer, parser and checker use. Only the shared emitter decorators and
// rustfmt adapt generated output to its place in the crate.
import { spawnSync } from 'node:child_process';
import { readFileSync, writeFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';

import { DecoratorSet } from '../src/decorators.js';
import { decorateEmitted } from '../src/grammar-emitters/common.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { parseJavaScript } from '../src/translation/javascript.js';

const sourcePath = new URL('../src/translation/frontend-rules.js', import.meta.url);
const outputPath = new URL('../../rust/src/translation/frontend_rules.rs', import.meta.url);
const decoratorsPath = new URL('../../parity/self-translation/frontend-rules-decorators.lino', import.meta.url);
const source = readFileSync(sourcePath, 'utf8');
const decorators = DecoratorSet.fromLino(readFileSync(decoratorsPath, 'utf8'));
const emitted = emitRust(checkProgram(parseJavaScript(source)));
const decorated = decorateEmitted('Rust', { source: emitted.text }, decorators).source;
const formatted = spawnSync('rustfmt', ['--edition', '2024', '--emit', 'stdout'], { input: decorated, encoding: 'utf8', maxBuffer: 4 * 1024 * 1024 });
if (formatted.error || formatted.status !== 0) throw formatted.error ?? new Error(formatted.stderr);
// Module-level style rules can replace a formatted expression spanning lines.
// The decorator engine and its Links Notation data are shared by both runtimes.
const styled = decorators.decorate('emitter', { format: 'Rust', scope: 'formatted-module', source: formatted.stdout });
const final = spawnSync('rustfmt', ['--edition', '2024', '--emit', 'stdout'], { input: styled.source, encoding: 'utf8', maxBuffer: 4 * 1024 * 1024 });
if (final.error || final.status !== 0) throw final.error ?? new Error(final.stderr);
const text = `// Generated from js/src/translation/frontend-rules.js by\n// js/scripts/generate-frontend-rules.mjs. Do not edit by hand.\n${final.stdout}`;
if (process.argv.includes('--check')) {
  if (readFileSync(outputPath, 'utf8') !== text) throw new Error('frontend rules are stale; run node js/scripts/generate-frontend-rules.mjs');
  console.log('generated Rust frontend rules match the JavaScript module');
} else {
  writeFileSync(outputPath, text);
  console.log(`wrote ${fileURLToPath(outputPath)}`);
}
