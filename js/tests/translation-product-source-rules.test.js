import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';
import { runInNewContext } from 'node:vm';

import { checkProgram } from '../src/translation/check.js';
import { DecoratorSet } from '../src/decorators.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { solveProductSourceFacts } from '../src/translation/product-source-rules.js';

const source = readFileSync(new URL('../src/translation/product-source-rules.js', import.meta.url), 'utf8');

test('alias evidence uses OR and propagates through multiple grounded source edges', () => {
  const bases = Object.freeze([true, false, false, false, false]);
  const targets = Object.freeze([2, 2, 3, 4]);
  const sources = Object.freeze([1, 0, 2, 3]);
  const conjunctions = Object.freeze([false, false, false, false, false]);
  assert.deepEqual(solveProductSourceFacts(bases, targets, sources, conjunctions), [true, false, true, true, true]);
  assert.deepEqual(bases, [true, false, false, false, false]);
  assert.deepEqual(targets, [2, 2, 3, 4]);
  assert.deepEqual(sources, [1, 0, 2, 3]);
});

test('every-caller evidence uses AND, requires a caller and waits for every caller to be proven', () => {
  const bases = [true, false, false, false];
  assert.deepEqual(solveProductSourceFacts(bases, [2, 2], [0, 1], [false, false, true, true]), [true, false, false, false]);
  assert.deepEqual(solveProductSourceFacts(bases, [1, 2, 2], [0, 0, 1], [false, false, true, true]), [true, true, true, false]);
  assert.deepEqual(solveProductSourceFacts([true, false], [], [], [true, true]), [true, false]);
});

test('ungrounded alias and caller cycles cannot manufacture immutable product evidence', () => {
  assert.deepEqual(solveProductSourceFacts([false, false], [0, 1], [1, 0], [false, false]), [false, false]);
  assert.deepEqual(solveProductSourceFacts([true, false, false], [1, 1, 2], [0, 2, 1], [false, true, false]), [true, false, false]);
  assert.deepEqual(solveProductSourceFacts([true, false, false], [1, 1, 2], [0, 2, 1], [false, false, false]), [true, true, true]);
});

test('malformed source edges fail as a whole before any graph evidence is propagated', () => {
  assert.deepEqual(solveProductSourceFacts([true], [], [], []), []);
  assert.deepEqual(solveProductSourceFacts([true], [0], [], [false]), []);
  for (const invalid of [-1, 0.5, 1, NaN, Infinity, 9007199254740992]) {
    assert.deepEqual(solveProductSourceFacts([true], [invalid], [0], [false]), [], `target ${invalid}`);
    assert.deepEqual(solveProductSourceFacts([true], [0], [invalid], [false]), [], `source ${invalid}`);
  }
  assert.deepEqual(solveProductSourceFacts([], [], [], []), []);
});

test('checked semantic emission retains source constraint outcomes without restoration provenance', () => {
  const checked = checkProgram(parseJavaScript(source, { referenceSemantics: false }));
  const emitted = emitJavaScript(checked);
  const context = {};
  runInNewContext(emitted.text, context);
  const cases = [
    [[true, false, false], [1, 2], [0, 1], [false, false, false], [true, true, true]],
    [[true, false, false], [2, 2], [0, 1], [false, false, true], [true, false, false]],
    [[false, false], [0, 1], [1, 0], [false, false], [false, false]],
    [[true], [NaN], [0], [false], []],
  ];
  for (const [bases, targets, sources, conjunctions, expected] of cases) {
    assert.deepEqual(Array.from(context.solveProductSourceFacts(bases, targets, sources, conjunctions)), expected);
  }
});

test('native solver keeps emitted helper dependencies and relocates exact generated declarations', () => {
  const emitted = emitRust(checkProgram(parseJavaScript(source, { referenceSemantics: false })));
  const raw = [...emitted.preludes, ...emitted.definitions].join('\n\n');
  assert.match(raw, /pub mod ml_math/u);
  assert.match(raw, /pub mod ml_array/u);
  assert.match(raw, /pub enum MlClosure1Loop2Result/u);
  const style = DecoratorSet.fromLino(readFileSync(new URL('../../parity/self-translation/product-source-rules-decorators.lino', import.meta.url), 'utf8'));
  const adapted = style.decorate('emitter', { format: 'Rust', source: raw }).source;
  assert.doesNotMatch(adapted, /\bcrate::/u);
  assert.match(adapted, /self::ml_math::is_safe_integer/u);
  assert.match(adapted, /self::ml_array::optional_at/u);
  assert.match(adapted, /self::MlClosure1Loop2Result/u);
  for (const decorator of style.decorators) {
    for (const action of decorator.actions) {
      assert.equal(action.op, 'replace');
      const name = action.from.slice('crate::'.length);
      assert.match(raw, new RegExp(`^pub (?:fn|enum|struct|mod) ${name}\\b`, 'mu'));
      assert.equal(action.to, `self::${name}`);
    }
  }
  const generated = readFileSync(new URL('../../rust/src/translation/javascript/product_source_rules.rs', import.meta.url), 'utf8');
  assert.match(generated, /Generated from js\/src\/translation\/product-source-rules\.js by the checked translator/u);
  assert.match(generated, /pub fn solve_product_source_facts/u);
  assert.match(generated, /pub mod ml_math/u);
  assert.match(generated, /pub mod ml_array/u);
  assert.doesNotMatch(generated, /\bcrate::/u);
});
