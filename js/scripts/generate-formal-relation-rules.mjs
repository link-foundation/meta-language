#!/usr/bin/env node
// Self-translate the shared relation admission decision; never compile locally.
import { spawnSync } from 'node:child_process';
import { readFileSync, writeFileSync } from 'node:fs';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { parseJavaScript } from '../src/translation/javascript.js';

const input = new URL('../src/formal-relation-rules.js', import.meta.url);
const output = new URL('../../rust/src/formal_concept_definition/relation_rules.rs', import.meta.url);
const translated = emitRust(checkProgram(parseJavaScript(readFileSync(input, 'utf8'))));
const formatted = spawnSync('rustfmt', ['--edition', '2024', '--emit', 'stdout'], { input: translated.text, encoding: 'utf8', maxBuffer: 4 * 1024 * 1024 });
if (formatted.error || formatted.status !== 0) throw formatted.error ?? new Error(formatted.stderr);
const text = '// Generated from js/src/formal-relation-rules.js by\n// js/scripts/generate-formal-relation-rules.mjs. Do not edit by hand.\n' + formatted.stdout;
if (process.argv.includes('--check')) {
  if (readFileSync(output, 'utf8') !== text) throw new Error('formal relation rules are stale; run node js/scripts/generate-formal-relation-rules.mjs');
  console.log('generated Rust formal relation rules match JavaScript');
} else writeFileSync(output, text);
