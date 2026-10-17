import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';
import { runInNewContext } from 'node:vm';

import { translateProgram } from '../src/program-translation.js';
import { selfTranslate } from '../src/self-translation.js';
import { DecoratorSet } from '../src/decorators.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { acceptRustSourceTextEquality } from '../src/translation/rust-indirection-rules.js';

test('owned String equality against a string slice retains valid original declaration and outcomes', () => {
  const source = `pub fn is_rust(name: String) -> bool { name == "Rust" || name == "rs" }
pub fn reversed(name: String) -> bool { "Rust" == name }
pub fn different(name: String) -> bool { name != "Rust" }
pub fn borrowed(name: &str) -> bool { String::from("Rust") == name }
pub fn converted() -> bool { String::from("Rust") == "Rust" }
pub fn projected(name: String) -> bool { String::from("Rust") == name.as_str() }
fn main() {}`;
  const translated = translateProgram(source, 'Rust', 'JavaScript');
  assert.equal(translated.diagnostic, null);
  const context = {};
  runInNewContext(translated.code, context);
  assert.equal(context.is_rust('Rust'), true);
  assert.equal(context.is_rust('rs'), true);
  assert.equal(context.is_rust('JavaScript'), false);
  assert.equal(context.reversed('Rust'), true);
  assert.equal(context.reversed('rs'), false);
  assert.equal(context.different('Rust'), false);
  assert.equal(context.different('rs'), true);
  assert.equal(context.borrowed('Rust'), true);
  assert.equal(context.borrowed('rs'), false);
  assert.equal(context.converted(), true);
  assert.equal(context.projected('Rust'), true);
});

test('original arithmetic edited self-translation fixture keeps its exact expected JavaScript', () => {
  const source = readFileSync(new URL('../../parity/self-translation/sources/arithmetic-edited.rs', import.meta.url), 'utf8');
  const expected = readFileSync(new URL('../../parity/self-translation/expected/arithmetic-edited-to-javascript.mjs', import.meta.url), 'utf8');
  const translated = selfTranslate(source, 'Rust', 'JavaScript');
  assert.equal(translated.code, expected);
  assert.ok(translated.items.some(item => item.term === 'function_item' && item.status === 'translated'));
});

test('text equality does not authorize boxed values, borrowed String owners or extra slice references', () => {
  const sources = [
    'fn invalid(name: Box<String>) -> bool { name == "Rust" } fn main() {}',
    'fn invalid(name: &String) -> bool { String::from("Rust") == name } fn main() {}',
    'fn invalid(name: & &str) -> bool { String::from("Rust") == name } fn main() {}',
    'fn invalid(name: Box<&str>) -> bool { String::from("Rust") == name } fn main() {}',
    'fn invalid(value: &u32) -> bool { value == 1u32 } fn main() {}',
  ];
  for (const source of sources) {
    const result = translateProgram(source, 'Rust', 'JavaScript');
    assert.equal(result.diagnostic?.kind, 'unsupported', source);
    assert.match(result.diagnostic.message, /source indirection/u, source);
  }
});

test('shared equality decision requires the actual direct String and borrowed slice pair', () => {
  for (const operation of ['eq', 'ne']) {
    assert.equal(acceptRustSourceTextEquality(operation, true, false, false, true), true);
    assert.equal(acceptRustSourceTextEquality(operation, false, true, true, false), true);
    assert.equal(acceptRustSourceTextEquality(operation, true, false, false, false), false);
    assert.equal(acceptRustSourceTextEquality(operation, false, false, true, true), false);
  }
  for (const operation of ['lt', 'le', 'add', 'and']) assert.equal(acceptRustSourceTextEquality(operation, true, false, false, true), false);
});

test('source classifier parenthesis style is scoped to the exact generated classifier', () => {
  const source = readFileSync(new URL('../src/translation/rust-indirection-rules.js', import.meta.url), 'utf8');
  const raw = emitRust(checkProgram(parseJavaScript(source))).definitions.join('\n\n');
  const styles = DecoratorSet.fromLino(readFileSync(new URL('../../parity/self-translation/rust-indirection-rules-decorators.lino', import.meta.url), 'utf8'));
  const normalized = styles.decorators.flatMap(decorator => decorator.actions).filter(action => action.from.startsWith('pub fn read_rust_source_expression_origin('));
  assert.equal(normalized.length, 1);
  assert.equal(normalized[0].op, 'replace');
  assert.equal(normalized[0].field, 'source');
  assert.ok(normalized[0].to.startsWith('pub fn read_rust_source_expression_origin('));
  assert.equal(normalized[0].from.replace(/[()]/gu, ''), normalized[0].to.replace(/[()]/gu, ''));
  assert.match(normalized[0].to, /kind == "toString" && \(operation == "from"\)/u);
  const adapted = styles.decorate('emitter', { format: 'Rust', source: raw }).source;
  assert.doesNotMatch(adapted, /^\s*if \(kind == "toString"/mu);
  const generated = readFileSync(new URL('../../rust/src/translation/rust/indirection_rules.rs', import.meta.url), 'utf8');
  assert.match(generated, /if kind == "toString" && \(operation == "from"\)/u);
  assert.doesNotMatch(generated, /clippy::(?:all|pedantic|nursery)\b/u);
});
