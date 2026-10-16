#!/usr/bin/env node
// Translate the JavaScript frontend decisions into the Rust module that the
// lexer, parser and checker use. Only the shared emitter decorators and
// rustfmt adapt generated output to its place in the crate.
import { spawnSync } from 'node:child_process';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';

import { DecoratorSet } from '../src/decorators.js';
import { decorateEmitted } from '../src/grammar-emitters/common.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { tokenize } from '../src/translation/lexer.js';
import { frontendRuleNativeStyleId, readFrontendRuleStyleCorrections } from './frontend-rule-style-corrections.mjs';

const sourcePath = new URL('../src/translation/frontend-rules.js', import.meta.url);
const outputPath = new URL('../../rust/src/translation/frontend_rules.rs', import.meta.url);
const decoratorsPath = new URL('../../parity/self-translation/frontend-rules-decorators.lino', import.meta.url);
const source = readFileSync(sourcePath, 'utf8');
const resultStyleId = 'reviewed-public-array-helper-results';
const updateArrayResults = process.argv.includes('--update-array-result-styles');
const updateNativeStyles = process.argv.includes('--update-reviewed-native-styles');
let decorators = DecoratorSet.fromLino(readFileSync(decoratorsPath, 'utf8'));
if (updateArrayResults) decorators = new DecoratorSet(decorators.decorators.filter(({ id }) => id !== resultStyleId));
if (updateNativeStyles) decorators = new DecoratorSet(decorators.decorators.filter(({ id }) => id !== frontendRuleNativeStyleId));
const emitted = emitRust(checkProgram(parseJavaScript(source)));
const decorated = decorateEmitted('Rust', { source: emitted.text }, decorators).source;
const formatted = spawnSync('rustfmt', ['--edition', '2024', '--emit', 'stdout'], { input: decorated, encoding: 'utf8', maxBuffer: 4 * 1024 * 1024 });
if (formatted.error || formatted.status !== 0) throw formatted.error ?? new Error(formatted.stderr);
// Module-level style rules can replace a formatted expression spanning lines.
// The decorator engine and its Links Notation data are shared by both runtimes.
let final = formatted;
for (let round = 0; round < 10; round += 1) {
  const styled = decorators.decorate('emitter', { format: 'Rust', scope: 'formatted-module', source: final.stdout });
  const next = spawnSync('rustfmt', ['--edition', '2024', '--emit', 'stdout'], { input: styled.source, encoding: 'utf8', maxBuffer: 4 * 1024 * 1024 });
  if (next.error || next.status !== 0) throw next.error ?? new Error(next.stderr);
  // Formatting a simplified block can expose another style replacement.
  // Require a fixed point so regeneration never depends on how many times
  // the command happened to run.
  if (next.stdout === final.stdout) break;
  final = next;
  if (round === 9) throw new Error('frontend style decorators do not converge');
}
if (updateNativeStyles) {
  const actions = readFrontendRuleStyleCorrections(final.stdout);
  const nativeStyles = { id: frontendRuleNativeStyleId, level: 'emitter', order: 3000,
    when: [['format', 'Rust'], ['scope', 'formatted-module']], actions };
  decorators = new DecoratorSet([...decorators.decorators, nativeStyles]);
  const previous = readFileSync(decoratorsPath, 'utf8').split('\n')
    .filter(line => !line.startsWith('(decorator ' + frontendRuleNativeStyleId + ' ')).join('\n');
  writeFileSync(decoratorsPath, previous + (previous.endsWith('\n') ? '\n' : '\n\n') + new DecoratorSet([nativeStyles]).toLino());
  const styled = decorators.decorate('emitter', { format: 'Rust', scope: 'formatted-module', source: final.stdout }).source;
  final = spawnSync('rustfmt', ['--edition', '2024', '--emit', 'stdout'], { input: styled, encoding: 'utf8', maxBuffer: 4 * 1024 * 1024 });
  if (final.error || final.status !== 0) throw final.error ?? new Error(final.stderr);
  if (decorators.decorate('emitter', { format: 'Rust', scope: 'formatted-module', source: final.stdout }).source !== final.stdout) {
    throw new Error('reviewed frontend native styles do not converge');
  }
}
if (updateArrayResults) {
  // Public runtime aliases expose these useful query and collection results.
  // Add attributes only; the checked operations and signatures remain exact.
  const start = final.stdout.indexOf('pub mod ml_array {');
  if (start < 0) throw new Error('the reviewed public array helper module is missing');
  const { tokens } = tokenize(final.stdout.slice(start), 'Rust');
  let depth = 0;
  let end = null;
  for (const token of tokens) {
    if (token.kind !== 'punct') continue;
    if (token.value === '{') depth += 1;
    if (token.value === '}' && --depth === 0) { end = start + token.end; break; }
  }
  if (end === null) throw new Error('the reviewed public array helper module is not balanced');
  const from = final.stdout.slice(start, end);
  const expected = ['optional_at', 'at', 'number_index', 'append', 'slice', 'index_of', 'includes', 'set', 'map'];
  const names = [...from.matchAll(/^\s*pub fn ([A-Za-z_][A-Za-z_0-9]*)\b/gmu)].map(match => match[1]);
  if (JSON.stringify(names) !== JSON.stringify(expected)) throw new Error('the checked public array operation inventory changed');
  const to = from.replace(/^( *)pub fn /gmu, '$1#[must_use]\n$1pub fn ');
  if (to.split('\n').filter(line => !/^\s*#\[must_use\]$/u.test(line)).join('\n') !== from) {
    throw new Error('array helper result styles changed more than attributes');
  }
  const resultStyle = { id: resultStyleId, level: 'emitter', order: 2000,
    when: [['format', 'Rust'], ['scope', 'formatted-module']], actions: [{ op: 'replace', field: 'source', from, to }] };
  decorators = new DecoratorSet([...decorators.decorators, resultStyle]);
  const previous = readFileSync(decoratorsPath, 'utf8').split('\n')
    .filter(line => !line.startsWith('(decorator ' + resultStyleId + ' ')).join('\n').trimEnd();
  writeFileSync(decoratorsPath, previous + '\n\n' + new DecoratorSet([resultStyle]).toLino());
  const styled = decorators.decorate('emitter', { format: 'Rust', scope: 'formatted-module', source: final.stdout }).source;
  final = spawnSync('rustfmt', ['--edition', '2024', '--emit', 'stdout'], { input: styled, encoding: 'utf8', maxBuffer: 4 * 1024 * 1024 });
  if (final.error || final.status !== 0) throw final.error ?? new Error(final.stderr);
  if (decorators.decorate('emitter', { format: 'Rust', scope: 'formatted-module', source: final.stdout }).source !== final.stdout) {
    throw new Error('array helper result styles do not converge');
  }
}
const notice = `// Generated from js/src/translation/frontend-rules.js by
// js/scripts/generate-frontend-rules.mjs. Do not edit by hand.
`;
// Keep the generated continuation machinery in a separate module so both
// generated Rust files obey the repository source-file size limit.
const boundary = final.stdout.search(/#\[derive\(Clone, Debug, PartialEq\)\]\npub enum Ml/u);
if (boundary < 0) throw new Error('generated continuation boundary is missing');
const strings = final.stdout.indexOf('pub fn read_string_map_operation(');
if (strings < 0 || strings >= boundary) throw new Error('generated string mapping boundary is missing');
const continuationLines = final.stdout.slice(boundary).trimEnd().split('\n');
const starts = [0];
for (let index = 1; index < continuationLines.length; index += 1) {
  if (!/^pub (?:fn|enum|struct|mod) /u.test(continuationLines[index])) continue;
  let start = index;
  while (start > starts.at(-1) && /^(?:\s*|\/\/\/.*|#\[[^\n]*\])$/u.test(continuationLines[start - 1])) start -= 1;
  if (start > starts.at(-1)) starts.push(start);
}
starts.push(continuationLines.length);
const continuations = [''];
for (let index = 0; index < starts.length - 1; index += 1) {
  const item = continuationLines.slice(starts[index], starts[index + 1]).join('\n');
  const current = continuations.at(-1);
  if (current && current.split('\n').length + item.split('\n').length > 900) continuations.push(item);
  else continuations[continuations.length - 1] = current ? `${current}\n${item}` : item;
}
const modules = continuations.map((_, index) => index === 0 ? 'continuations' : `continuations_${index + 1}`);
const outputs = [
  [outputPath, notice + final.stdout.slice(0, strings) + '#[path = "frontend_rules/strings.rs"]\nmod strings;\npub use strings::*;\n#[path = "frontend_rules/rust_strings.rs"]\nmod rust_strings;\npub use rust_strings::*;\n' + modules.map((name) => `#[path = "frontend_rules/${name}.rs"]\nmod ${name};\npub use ${name}::*;\n`).join('')],
  [new URL('../../rust/src/translation/frontend_rules/strings.rs', import.meta.url), notice + 'use super::*;\n\n' + final.stdout.slice(strings, boundary).trimEnd() + '\n'],
  ...modules.map((name, index) => [new URL(`../../rust/src/translation/frontend_rules/${name}.rs`, import.meta.url), notice + 'use super::*;\n\n' + continuations[index].trim() + '\n']),
];
if (process.argv.includes('--check')) {
  for (const [path, text] of outputs) {
    if (readFileSync(path, 'utf8') !== text) throw new Error('frontend rules are stale; run node js/scripts/generate-frontend-rules.mjs');
  }
  console.log('generated Rust frontend rules match the JavaScript module');
} else {
  mkdirSync(new URL('../../rust/src/translation/frontend_rules/', import.meta.url), { recursive: true });
  for (const [path, text] of outputs) writeFileSync(path, text);
  console.log(`wrote ${outputs.length} generated frontend modules`);
}
