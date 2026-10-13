#!/usr/bin/env node
// Generate structural admission from the JavaScript policy, without compiling.
import { spawnSync } from 'node:child_process';
import { readFileSync, writeFileSync } from 'node:fs';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { parseJavaScript } from '../src/translation/javascript.js';

const input = new URL('../src/concept-view-rules.js', import.meta.url);
const output = new URL('../../rust/src/concept_ontology/view_rules.rs', import.meta.url);
const translated = emitRust(checkProgram(parseJavaScript(readFileSync(input, 'utf8'))));
const formatted = spawnSync('rustfmt', ['--edition', '2024', '--emit', 'stdout'], { input: translated.text, encoding: 'utf8', maxBuffer: 4 * 1024 * 1024 });
if (formatted.error || formatted.status !== 0) throw formatted.error ?? new Error(formatted.stderr);
const text = '// Generated from js/src/concept-view-rules.js by\n// js/scripts/generate-concept-view-rules.mjs. Do not edit by hand.\n' + formatted.stdout;
if (process.argv.includes('--check')) {
  if (readFileSync(output, 'utf8') !== text) throw new Error('concept view rules are stale; run node js/scripts/generate-concept-view-rules.mjs');
  console.log('generated concept view rules match JavaScript');
} else writeFileSync(output, text);
