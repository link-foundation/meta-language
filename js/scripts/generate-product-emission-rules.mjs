#!/usr/bin/env node
import { spawnSync } from 'node:child_process';
import { readFileSync, writeFileSync } from 'node:fs';
import { DecoratorSet } from '../src/decorators.js';
import { decorateEmitted } from '../src/grammar-emitters/common.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { parseJavaScript } from '../src/translation/javascript.js';

const input = new URL('../src/translation/product-emission-rules.js', import.meta.url);
const output = new URL('../../rust/src/translation/product_emission_rules.rs', import.meta.url);
const decorators = DecoratorSet.fromLino(readFileSync(new URL('../../parity/self-translation/product-emission-rules-decorators.lino', import.meta.url), 'utf8'));
const translated = emitRust(checkProgram(parseJavaScript(readFileSync(input, 'utf8'))));
const adapted = decorateEmitted('Rust', { source: translated.text }, decorators).source;
const formatted = spawnSync('rustfmt', ['--edition', '2024', '--emit', 'stdout'], { input: adapted, encoding: 'utf8', maxBuffer: 4 * 1024 * 1024 });
if (formatted.error || formatted.status !== 0) throw formatted.error ?? new Error(formatted.stderr);
const text = '// Generated from js/src/translation/product-emission-rules.js by\n// js/scripts/generate-product-emission-rules.mjs. Do not edit by hand.\n' + formatted.stdout;
if (process.argv.includes('--check')) {
  if (readFileSync(output, 'utf8') !== text) throw new Error('product emission rules are stale; run node js/scripts/generate-product-emission-rules.mjs');
  console.log('generated Rust product emission rules match JavaScript');
} else writeFileSync(output, text);
