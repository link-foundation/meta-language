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
  const header = emitted.text.split('\n').find(line => line.startsWith('#![allow('));
  const raw = [header, ...emitted.preludes, ...emitted.definitions].join('\n\n');
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
    if (decorator.id === 'shared-product-source-parameter-loans') {
      assert.deepEqual(decorator.when, [['format', 'Rust'], ['scope', 'formatted-product-source-parameters']]);
      assert.equal(decorator.actions.length, 1);
      const action = decorator.actions[0];
      assert.equal(action.op, 'replace');assert.equal(action.field, 'source');
      assert.match(action.from, /^pub fn accept_java_script_array_tuple_context\(/u);
      assert.equal(action.to, action.from.replace('source_kind: String', 'source_kind: &str')
        .replace('expected_kind: String', 'expected_kind: &str'));
      assert.equal(action.to.slice(action.to.indexOf('{')), action.from.slice(action.from.indexOf('{')),
        'the complete contextual-array decision body remains byte exact');
      continue;
    }
    if (decorator.id === 'reviewed-product-source-condition-lints-26667280') {
      assert.deepEqual(decorator.when, [['format', 'Rust'], ['scope', 'formatted-product-source-conditions']]);
      assert.equal(decorator.actions.length, 1);
      const action = decorator.actions[0];
      assert.equal(action.op, 'replace');assert.equal(action.field, 'source');
      assert.match(action.from, /^pub fn solve_product_source_facts\(/u);
      assert.equal([...action.from.matchAll(/if \(\{/gu)].length, 2);
      assert.doesNotMatch(action.to, /if \(\{|#\[allow|mul_add/u);
      assert.equal([...action.to.matchAll(/let ml_product_condition_/gu)].length, 2);
      continue;
    }
    if (decorator.id === 'reviewed-product-native-lints-285c8b13') {
      assert.deepEqual(decorator.when, [['format', 'Rust'], ['scope', 'formatted-product-library']]);
      assert.ok(decorator.actions.every(action => action.op === 'replace' && action.field === 'source'));
      assert.ok(decorator.actions.some(action => action.from.startsWith('pub mod ml_array')
        && action.to === 'pub use crate::translation::frontend_rules::ml_array;'));
      continue;
    }
    for (const action of decorator.actions) {
      assert.equal(action.op, 'replace');
      if (action.from === header) {
        assert.equal(action.to, header.replace(')]', ', clippy::float_cmp, clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]'));
        assert.match(raw, /pub mod ml_math/u);
        assert.match(raw, /pub mod ml_array/u);
        assert.match(raw, /Some\(index as usize\)/u);
        continue;
      }
      if (action.from.startsWith('pub fn ')) {
        const name = action.from.slice('pub fn '.length);
        const parameters = new RegExp(`^pub fn ${name}\\(([^)]*)\\)`, 'mu').exec(raw)?.[1];
        assert.ok(parameters);
        assert.ok(parameters.split(',').length > 7);
        assert.equal(action.to, `// Checked closure lowering passes its captured state explicitly.\n#[allow(clippy::too_many_arguments)]\npub fn ${name}`);
        continue;
      }
      if (/^\d+\.0$/u.test(action.from)) {
        assert.ok(raw.includes(action.from));
        assert.ok(['9007199254740991.0', '9007199254740992.0'].includes(action.from));
        assert.equal(action.to.replaceAll('_', ''), action.from);
        continue;
      }
      const name = action.from.slice('crate::'.length);
      assert.match(raw, new RegExp(`^pub (?:fn|enum|struct|mod) ${name}\\b`, 'mu'));
      assert.equal(action.to, `self::${name}`);
    }
  }
  const generated = readFileSync(new URL('../../rust/src/translation/javascript/product_source_rules.rs', import.meta.url), 'utf8');
  assert.match(generated, /Generated from js\/src\/translation\/product-source-rules\.js by the checked translator/u);
  assert.match(generated, /pub fn solve_product_source_facts/u);
  assert.match(generated, /pub fn accept_java_script_array_tuple_context\(\s*source_kind: &str,\s*expected_kind: &str,/u);
  assert.match(generated, /pub mod ml_math/u);
  assert.match(generated, /pub use crate::translation::frontend_rules::ml_array;/u);
  for (const allowance of ['float_cmp', 'cast_precision_loss', 'cast_possible_truncation', 'cast_sign_loss']) assert.match(generated, new RegExp(`clippy::${allowance}\\b`, 'u'));
  assert.match(generated, /#\[allow\(clippy::too_many_arguments\)\]\n#\[must_use\]\npub fn ml_closure1_loop2/u);
  assert.doesNotMatch(generated, /clippy::(?:all|pedantic|nursery)\b/u);
  assert.match(generated, /9_007_199_254_740_991\.0/u);
  const arrayProvider = readFileSync(new URL('../../rust/src/translation/frontend_rules.rs', import.meta.url), 'utf8');
  assert.match(arrayProvider, /9_007_199_254_740_992\.0/u);
  assert.doesNotMatch(generated.replace('pub use crate::translation::frontend_rules::ml_array;', ''), /\bcrate::/u);
});
