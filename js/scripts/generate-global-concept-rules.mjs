#!/usr/bin/env node
// Translate the global concept correspondence decision once from JavaScript.
// Formatting and source adaptations are reproducible shared Links decorators.
import { spawnSync } from 'node:child_process';
import { readFileSync, writeFileSync } from 'node:fs';

import { DecoratorSet } from '../src/decorators.js';
import { decorateEmitted } from '../src/grammar-emitters/common.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { parseJavaScript } from '../src/translation/javascript.js';

const sourcePath = new URL('../src/global-concept-rules.js', import.meta.url);
const outputPath = new URL('../../rust/src/global_concept_rules.rs', import.meta.url);
const decoratorsPath = new URL('../../parity/self-translation/global-concept-rules-decorators.lino', import.meta.url);
const decorators = DecoratorSet.fromLino(readFileSync(decoratorsPath, 'utf8'));
const program = checkProgram(parseJavaScript(readFileSync(sourcePath, 'utf8')));
const emitted = decorateEmitted('Rust', { source: emitRust(program).text }, decorators).source;
const formatted = spawnSync('rustfmt', ['--edition', '2024', '--emit', 'stdout'], { input: emitted, encoding: 'utf8', maxBuffer: 4 * 1024 * 1024 });
if (formatted.error || formatted.status !== 0) throw formatted.error ?? new Error(formatted.stderr);
let final = formatted.stdout;
for (let round = 0; round < 10; round += 1) {
  const styled = decorators.decorate('emitter', { format: 'Rust', scope: 'formatted-module', source: final });
  const next = spawnSync('rustfmt', ['--edition', '2024', '--emit', 'stdout'], { input: styled.source, encoding: 'utf8', maxBuffer: 4 * 1024 * 1024 });
  if (next.error || next.status !== 0) throw next.error ?? new Error(next.stderr);
  if (next.stdout === final) break;
  final = next.stdout;
  if (round === 9) throw new Error('global concept style decorators do not converge');
}
const text = '// Generated from js/src/global-concept-rules.js by\n// js/scripts/generate-global-concept-rules.mjs. Do not edit by hand.\n' + final;
if (process.argv.includes('--check')) {
  if (readFileSync(outputPath, 'utf8') !== text) throw new Error('global concept rules are stale; run node js/scripts/generate-global-concept-rules.mjs');
  console.log('generated Rust global concept rules match the JavaScript module');
} else {
  writeFileSync(outputPath, text);
  console.log('wrote generated Rust global concept rules');
}
