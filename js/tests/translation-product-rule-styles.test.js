import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import vm from 'node:vm';
import { DecoratorSet } from '../src/decorators.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { parseRust } from '../src/translation/rust.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { decorateEmitted } from '../src/grammar-emitters/common.js';
import { formatProductRuleSource, readProductRuleStyleCorrections, readProductSourceConditionStyleCorrections } from '../scripts/product-rule-style-corrections.mjs';

const provider = readFileSync(new URL('../../rust/src/translation/frontend_rules.rs', import.meta.url), 'utf8');
const styleId = 'reviewed-product-native-lints-285c8b13';
const sources = new Map();
function source(family) {
  if (sources.has(family)) return sources.get(family);
  const input = readFileSync(new URL(`../src/translation/product-${family}-rules.js`, import.meta.url), 'utf8');
  const emitted = emitRust(checkProgram(parseJavaScript(input, { referenceSemantics: false })));
  const styles = DecoratorSet.fromLino(readFileSync(new URL(`../../parity/self-translation/product-${family}-rules-decorators.lino`, import.meta.url), 'utf8'));
  const base = new DecoratorSet(styles.decorators.filter(definition => definition.id !== styleId));
  const unformatted = family === 'emission' ? decorateEmitted('Rust', { source: emitted.text }, base).source
    : base.decorate('emitter', { format: 'Rust', source: [emitted.text.split('\n').find(line => line.startsWith('#![allow(')), ...emitted.preludes, ...emitted.definitions].join('\n\n') }).source;
  const result = { raw: formatProductRuleSource(unformatted), styles };
  sources.set(family, result);
  return result;
}

test('array provider reuse keeps every existing operation and rejects any changed signature or body', () => {
  const { raw } = source('emission');
  const action = readProductRuleStyleCorrections(raw, 'emission', provider)[0];
  assert.equal(action.to, 'pub use crate::translation::frontend_rules::ml_array;');
  for (const name of ['optional_at', 'at', 'number_index', 'append', 'slice', 'index_of', 'includes', 'set', 'map']) {
    assert.match(action.from, new RegExp(`pub fn ${name}\\b`, 'u'));
    assert.match(provider, new RegExp(`pub fn ${name}\\b`, 'u'));
  }
  for (const changed of [provider.replace('index >= 0.0', 'index > 0.0'), provider.replace('values: &[T]', 'values: &mut [T]'), provider.replace('array index out of range', 'array  index out of range')]) {
    assert.throws(() => readProductRuleStyleCorrections(raw, 'emission', changed), /differs in operation, signature or body/u);
  }
});

test('reviewed styles retain full owning-function contexts and introduce no fused arithmetic or broad allowance', () => {
  for (const family of ['source', 'emission']) {
    const { styles } = source(family);
    const reviewed = styles.decorators.find(definition => definition.id === styleId);
    assert.ok(reviewed);
    for (const action of reviewed.actions) {
      if (action.from.startsWith('pub mod ml_array') || action.from.startsWith('/// Math.sign')) continue;
      assert.match(action.from, /^(?:pub )?fn [a-z_0-9]+\(/u);
      assert.ok(action.from.trimEnd().endsWith('}'));
      assert.doesNotMatch(action.to, /mul_add|#\[allow\(/u);
    }
  }
  const generated = readFileSync(new URL('../../rust/src/translation/product_emission_rules.rs', import.meta.url), 'utf8');
  assert.equal([...generated.matchAll(/#\[expect\(/gu)].length, 2);
  assert.equal([...generated.matchAll(/clippy::cast_precision_loss/gu)].length, 2);
  assert.match(generated, /2\^32 - 1/u);
  assert.doesNotMatch(generated, /clippy::(?:all|pedantic|nursery|cast_sign_loss|cast_possible_truncation)/u);
});

test('Boolean branch inversion and else-if flattening preserve both branches and unordered Number observations', () => {
  const { raw } = source('emission');
  const prefix = raw.slice(0, raw.indexOf('pub fn join_product_items'));
  const samples = [
    ['pub fn decide(value: bool) -> bool { if !value { false } else { true } }', [false, true]],
    ['pub fn decide(value: f64) -> f64 { if value < 0.0 { 1.0 } else { if value < 1.0 { 2.0 } else { 3.0 } } }', [-Infinity, -1, -0, 0, 0.5, 1, Infinity, NaN]],
  ];
  for (const [before, values] of samples) {
    const action = readProductRuleStyleCorrections(prefix + before, 'source', provider).find(action => action.from === before);
    assert.ok(action);
    const read = code => {
      const surface = parseRust(`${code} fn main(){println!("x");}`);
      const emitted = emitJavaScript(checkProgram(surface)).text;
      const context = { console: { log() {} } };
      vm.runInNewContext(emitted, context);
      return context.decide;
    };
    const original = read(before), transformed = read(action.to);
    for (const value of values) assert.ok(Object.is(original(value), transformed(value)), String(value));
  }
});

test('tail branch removal requires every then path to continue and preserves conditional evaluation once', () => {
  const { raw } = source('emission');
  const prefix = raw.slice(0, raw.indexOf('pub fn join_product_items'));
  const continuing = 'pub fn choose(value: bool) -> bool { loop { return if value { if value { continue; } else { continue; } } else { false }; } }';
  const fallingThrough = 'pub fn choose(value: bool) -> bool { loop { return if value { if value { continue; } else { false } } else { true }; } }';
  const nestedLoop = 'pub fn choose(value: bool) -> bool { loop { return if value { loop { if value { break true; } continue; } } else { false }; } }';
  const style = input => readProductRuleStyleCorrections(prefix + input, 'source', provider).find(action => action.from === input).to;
  assert.doesNotMatch(style(continuing), /return if value/u);
  assert.match(style(fallingThrough), /return if value/u);
  assert.match(style(nestedLoop), /return if value/u, 'a continuation of a nested loop is not a continuation of the owning loop');
  assert.equal([...style(continuing).matchAll(/if value/gu)].length, 2);
});

test('two emitted loop expectations retain the exact JavaScript array-length and index domain witnesses', () => {
  for (const length of [0, 1, 2, 31, 65536, 0xffff_ffff]) {
    assert.equal(Number(BigInt(length)), length);
    assert.ok(Number.isSafeInteger(length));
    for (const index of [0, Math.max(0, length - 1), length]) {
      assert.ok(Number.isSafeInteger(index));
      assert.equal(Number(BigInt(index) + 1n), index + 1);
      if (index < length) assert.ok(index >= 0 && index < 0x1_0000_0000);
    }
  }
  const { raw } = source('emission');
  assert.match(raw, /let index = 0f64/u);
  assert.match(raw, /index \+ 1f64/u);
  assert.match(raw, /index < \(items\.len\(\) as f64\)/u);
  assert.match(raw, /index < \(fields\.len\(\) as f64\)/u);
});

test('CI18 callback condition bindings preserve branch scope, evaluation order and local name hygiene', () => {
  const before = 'fn take_first(value:bool)->bool{value} fn take_second(value:bool)->bool{value} pub fn choose(outer:bool,first:bool,second:bool)->bool{let ml_product_condition_1=outer;if !outer{false}else if ({let local=take_first(first);local}){if ({let local=take_second(second);local}){true}else{false}}else{false}}';
  const actions=readProductSourceConditionStyleCorrections(before);
  assert.equal(actions.length,1);assert.match(actions[0].from,/^pub fn choose\(/u);
  assert.doesNotMatch(actions[0].to,/if \(\{/u);assert.match(actions[0].to,/let ml_product_condition_2/u);assert.match(actions[0].to,/let ml_product_condition_3/u);
  const after=before.replace(actions[0].from,actions[0].to);
  const read=source=>{
    const code=emitJavaScript(checkProgram(parseRust(source+' fn main(){}'))).text;
    const counter=[],context={counter};
    vm.runInNewContext(code.replace('function take_first(value) {','function take_first(value) { counter.push("first");').replace('function take_second(value) {','function take_second(value) { counter.push("second");'),context,{timeout:1000});
    return {counter,call:context.choose};
  };
  for(const outer of[false,true])for(const first of[false,true])for(const second of[false,true]){
    const original=read(before),transformed=read(after),expected=outer&&first&&second;
    assert.equal(original.call(outer,first,second),expected);assert.equal(transformed.call(outer,first,second),expected);
    const order=outer?(first?['first','second']:['first']):[];
    assert.deepEqual(original.counter,order);assert.deepEqual(transformed.counter,order);
  }
  const {styles}=source('source'),record=styles.decorators.find(decorator=>decorator.id==='reviewed-product-source-condition-lints-26667280');
  assert.equal(record.actions.length,1);assert.deepEqual(record.when,[['format','Rust'],['scope','formatted-product-source-conditions']]);
  assert.match(record.actions[0].from,/^pub fn solve_product_source_facts\(/u);assert.doesNotMatch(record.actions[0].to,/if \(\{/u);
});
