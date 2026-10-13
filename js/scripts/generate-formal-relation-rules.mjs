#!/usr/bin/env node
// Self-translate the shared relation admission decision; never compile locally.
import { spawnSync } from 'node:child_process';
import { readFileSync, writeFileSync } from 'node:fs';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { DecoratorSet } from '../src/decorators.js';

const input = new URL('../src/formal-relation-rules.js', import.meta.url);
const output = new URL('../../rust/src/formal_concept_definition/relation_rules.rs', import.meta.url);
const style = new URL('../../parity/self-translation/formal-relation-rules-decorators.lino', import.meta.url);
const definitions = emitRust(checkProgram(parseJavaScript(readFileSync(input, 'utf8')))).definitions;
const raw = definitions.join('\n\n');
if (process.argv.includes('--update-styles')) {
  let styled = raw;
  const actions = [];
  const replace = (from, to) => {
    if (!styled.includes(from)) throw new Error(`a formal relation style target is absent: ${from}`);
    actions.push({ op: 'replace', field: 'source', from, to });
    styled = styled.split(from).join(to);
  };
  for (const parameter of ['label_kind', 'label_definition', 'relation_kind']) replace(`${parameter}: String`, `${parameter}: &str`);
  for (const definition of definitions) {
    const name = /^pub fn (\w+)/u.exec(definition)?.[1];
    const expression = /\{\n\s+([^\n]+)\n\}$/u.exec(definition)?.[1];
    const scalarTests = expression?.replace(/"(?:[^"\\]|\\.)*"/gu, '""');
    // These read-only decisions contain only a conjunction of scalar tests.
    // Keep a changed operator or parentheses inside a literal for review.
    if (!name || !expression || /[^\s()!&=\w"]/u.test(scalarTests) || /!\s*\(/u.test(expression)
      || [...expression.matchAll(/"(?:[^"\\]|\\.)*"/gu)].some(match => /[()]/u.test(match[0]))) {
      throw new Error('the formal relation decision needs a reviewed source style');
    }
    replace(expression, expression.replace(/[()]/gu, ''));
    replace(`pub fn ${name}`, `/// Admit the existing formal labeled relation model.\n#[must_use]\npub fn ${name}`);
  }
  // Positive, finite, nonzero binary64 3 has one encoding. Bit equality is
  // therefore exactly its Number equality predicate, including NaN and zeros.
  replace('relation_arity == 3f64', 'relation_arity.to_bits() == 3.0_f64.to_bits()');
  writeFileSync(style, new DecoratorSet([{ id: 'formal-relation-library-style', level: 'emitter', order: 0,
    when: [['format', 'Rust']], actions }]).toLino());
}
const translated = DecoratorSet.fromLino(readFileSync(style, 'utf8')).decorate('emitter', { format: 'Rust', source: raw }).source;
const formatted = spawnSync('rustfmt', ['--edition', '2024', '--emit', 'stdout'], { input: translated, encoding: 'utf8', maxBuffer: 4 * 1024 * 1024 });
if (formatted.error || formatted.status !== 0) throw formatted.error ?? new Error(formatted.stderr);
const text = '// Generated from js/src/formal-relation-rules.js by\n// js/scripts/generate-formal-relation-rules.mjs. Do not edit by hand.\n'
  + '// Emitter styles: parity/self-translation/formal-relation-rules-decorators.lino.\n' + formatted.stdout;
if (process.argv.includes('--check')) {
  if (readFileSync(output, 'utf8') !== text) throw new Error('formal relation rules are stale; run node js/scripts/generate-formal-relation-rules.mjs');
  console.log('generated Rust formal relation rules match JavaScript');
} else writeFileSync(output, text);
