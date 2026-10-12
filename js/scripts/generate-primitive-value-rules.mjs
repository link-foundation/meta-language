#!/usr/bin/env node
// Generate executable primitive decisions from the JavaScript implementation.
import { spawnSync } from 'node:child_process';
import { readFileSync, writeFileSync } from 'node:fs';

import { DecoratorSet } from '../src/decorators.js';
import { decorateEmitted } from '../src/grammar-emitters/common.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { parseJavaScript } from '../src/translation/javascript.js';

const input = new URL('../src/translation/primitive-value-rules.js', import.meta.url);
const output = new URL('../../rust/src/translation/primitive_value_rules.rs', import.meta.url);
const decorators = DecoratorSet.fromLino(readFileSync(new URL('../../parity/self-translation/primitive-value-rules-decorators.lino', import.meta.url), 'utf8'));
const translated = emitRust(checkProgram(parseJavaScript(readFileSync(input, 'utf8'))));
const adapted = decorateEmitted('Rust', { source: translated.text }, decorators).source;
const formatted = spawnSync('rustfmt', ['--edition', '2024', '--emit', 'stdout'], { input: adapted, encoding: 'utf8', maxBuffer: 4 * 1024 * 1024 });
if (formatted.error || formatted.status !== 0) throw formatted.error ?? new Error(formatted.stderr);
const text = '// Generated from js/src/translation/primitive-value-rules.js by\n// js/scripts/generate-primitive-value-rules.mjs. Do not edit by hand.\n' + formatted.stdout;
if (process.argv.includes('--check')) {
  if (readFileSync(output, 'utf8') !== text) throw new Error('primitive value rules are stale; run node js/scripts/generate-primitive-value-rules.mjs');
  console.log('generated Rust primitive value rules match JavaScript');
} else writeFileSync(output, text);
