#!/usr/bin/env node
// Generate executable primitive decisions from the JavaScript implementation.
import { spawnSync } from 'node:child_process';
import { readFileSync, writeFileSync } from 'node:fs';

import { DecoratorSet } from '../src/decorators.js';
import { decorateEmitted } from '../src/grammar-emitters/common.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { tokenize } from '../src/translation/lexer.js';

const input = new URL('../src/translation/primitive-value-rules.js', import.meta.url);
const output = new URL('../../rust/src/translation/primitive_value_rules.rs', import.meta.url);
const styleFile = new URL('../../parity/self-translation/primitive-value-rules-decorators.lino', import.meta.url);
let decorators = DecoratorSet.fromLino(readFileSync(styleFile, 'utf8'));
const translated = emitRust(checkProgram(parseJavaScript(readFileSync(input, 'utf8'))));
// This is a library of decisions. Its executable harness and lint preamble do
// not belong in the crate; every translated public definition remains present.
const raw = translated.definitions.join('\n\n');
if (process.argv.includes('--update-library-styles')) {
  const id = 'reviewed-ci-fourteen-primitive-library-style';
  const previous = new DecoratorSet(decorators.decorators.filter(record => record.id !== id));
  let source = decorateEmitted('Rust', { source: raw }, previous).source;
  const actions = [];
  const replace = (from, to) => {
    if (!source.includes(from)) throw new Error(`reviewed primitive style target is absent: ${from}`);
    actions.push({ op: 'replace', field: 'source', from, to });
    source = source.split(from).join(to);
  };
  for (const name of ['name', 'operation', 'absence', 'target', 'language', 'kind', 'expression']) {
    replace(`${name}: String`, `${name}: &str`);
  }
  replace('operation.clone()', 'operation');
  replace('absence.clone()', 'absence');
  replace('String::from("")', 'String::new()');
  replace('value == ""', 'value.is_empty()');
  // Inclusive contains executes the same two ordered binary64 comparisons.
  // NaN fails both forms; either zero encoding remains accepted. No rounding
  // operation, integer cast or value-domain restriction is introduced.
  replace('((index >= (0f64)) && (index <= (9_007_199_254_740_991_f64)))',
    '(0.0_f64..=9_007_199_254_740_991_f64).contains(&index)');
  for (;;) {
    const match = /\} else \{\n\s+if /u.exec(source);
    if (!match) break;
    const opening = match.index + '} else '.length;
    let depth = 1; let cursor = opening + 1; let quoted = false;
    for (; cursor < source.length && depth; cursor += 1) {
      const character = source[cursor];
      if (quoted && character === '\\') { cursor += 1; continue; }
      if (character === '"') quoted = !quoted;
      if (!quoted && character === '{') depth += 1;
      if (!quoted && character === '}') depth -= 1;
    }
    if (depth) throw new Error('a primitive branch is unclosed');
    replace(source.slice(match.index, cursor), `} else ${source.slice(opening + 1, cursor - 1).trim()}`);
  }
  const { tokens } = tokenize(source, 'Rust');
  const openings = []; const closing = new Map();
  for (const [index, token] of tokens.entries()) {
    if (token.kind === 'punct' && token.value === '(') openings.push(index);
    else if (token.kind === 'punct' && token.value === ')') closing.set(openings.pop(), index);
  }
  if (openings.length || closing.has(undefined)) throw new Error('primitive grouping is unbalanced');
  const removed = new Set();
  for (const [index, token] of tokens.entries()) {
    if (!['if', '{'].includes(token.value) || tokens[index + 1]?.value !== '(') continue;
    let start = index + 1; let end = closing.get(start);
    if (end === undefined || tokens[end + 1]?.value !== (token.value === 'if' ? '{' : '}')) continue;
    do {
      removed.add(start); removed.add(end); start += 1; end -= 1;
    } while (tokens[start]?.value === '(' && closing.get(start) === end);
  }
  // Remove scalar wrappers and Boolean grouping only when the surrounding
  // precedence permits it. Calls, receivers and arithmetic grouping stay intact.
  for (const [start, end] of closing) {
    const before = tokens[start - 1]; const after = tokens[end + 1];
    if ((before?.kind === 'identifier' && before.value !== 'if')
      || ['.', '::', '[', '('].includes(after?.value) || before?.value === '!') continue;
    const inside = tokens.slice(start + 1, end);
    const scalar = token => ['identifier', 'number', 'string'].includes(token?.kind);
    const atom = inside.length === 1 && scalar(inside[0]);
    const negation = inside.length === 2 && inside[0].value === '!' && scalar(inside[1]);
    const comparison = inside.length === 3 && scalar(inside[0]) && scalar(inside[2])
      && ['==', '!=', '<', '>', '<=', '>='].includes(inside[1].value);
    let depth = 0; const operators = [];
    for (const token of inside) {
      if (token.kind === 'punct' && token.value === '(') depth += 1;
      else if (token.kind === 'punct' && token.value === ')') depth -= 1;
      else if (!depth && ['&&', '||', '==', '!=', '<', '>', '<=', '>='].includes(token.value)) operators.push(token.value);
    }
    const precedence = operators.includes('||') ? 1 : operators.includes('&&') ? 2 : operators.length ? 3 : 0;
    const required = Math.max(...[before, after].map(token => token?.value === '&&' ? 2 : token?.value === '||' ? 1 : 0));
    if (atom || negation || comparison || (precedence && precedence >= required)) {
      removed.add(start); removed.add(end);
    }
  }
  let normalized = source;
  for (const index of [...removed].sort((first, second) => second - first)) {
    const token = tokens[index];
    normalized = normalized.slice(0, token.start) + normalized.slice(token.end);
  }
  if (normalized !== source) replace(source, normalized);
  for (const match of raw.matchAll(/^pub fn (\w+)/gmu)) {
    replace(`pub fn ${match[1]}`, `/// Apply the shared primitive value decision.\n#[must_use]\npub fn ${match[1]}`);
  }
  decorators = new DecoratorSet([...previous.decorators, { id, level: 'emitter', order: 2000, when: [['format', 'Rust']], actions }]);
  writeFileSync(styleFile, decorators.toLino());
}
const lines = decorateEmitted('Rust', { source: raw }, decorators).source;
const adapted = decorators.decorate('emitter', { format: 'Rust', source: lines }).source;
const formatted = spawnSync('rustfmt', ['--edition', '2024', '--emit', 'stdout'], { input: adapted, encoding: 'utf8', maxBuffer: 4 * 1024 * 1024 });
if (formatted.error || formatted.status !== 0) throw formatted.error ?? new Error(formatted.stderr);
const text = '// Generated from js/src/translation/primitive-value-rules.js by\n// js/scripts/generate-primitive-value-rules.mjs. Do not edit by hand.\n' + formatted.stdout;
if (process.argv.includes('--check')) {
  if (readFileSync(output, 'utf8') !== text) throw new Error('primitive value rules are stale; run node js/scripts/generate-primitive-value-rules.mjs');
  console.log('generated Rust primitive value rules match JavaScript');
} else writeFileSync(output, text);
