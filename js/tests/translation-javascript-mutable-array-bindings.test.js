import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { runInNewContext } from 'node:vm';
import { parseJavaScript } from '../src/translation/javascript.js';
import { acceptJavaScriptMutableArrayBinding, acceptJavaScriptMutableArrayBindingOwner } from '../src/translation/frontend-rules.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { LinkNetwork } from '../src/network.js';
import { encodeLinkNetworkBinary, decodeLinkNetworkBinary } from '../src/link-network-binary.js';
import { encodeTypedProgramLinks, decodeTypedProgramLinks } from '../src/typed-program-links.js';
import { recheckLinkedTypedProgram } from '../src/typed-program-checked-projection.js';
import { selfTranslate } from '../src/self-translation.js';
import { JAVASCRIPT_ARRAY_BINDING_POLICY } from '../src/translation/array-binding-policy.js';
import { DecoratorSet } from '../src/decorators.js';
import { decorateEmitted } from '../src/grammar-emitters/common.js';
import { generateJavaScriptMutableArrayBindings, javaScriptMutableArrayBindingStyleId, readJavaScriptMutableArrayBindingStyles } from '../scripts/generate-javascript-mutable-array-bindings.mjs';
import { formatProductRuleSource } from '../scripts/product-rule-style-corrections.mjs';
import { readRustLibraryItems } from '../scripts/rust-library-packaging.mjs';

function observe(source) {
  const lines = [];
  try {
    runInNewContext(source, { console: { log: (...values) => lines.push(values.map(String).join(' ')) } }, { timeout: 1000 });
    return { lines, error: null };
  } catch (error) {
    return { lines, error: { name: error.name, message: error.message } };
  }
}

function equivalent(source, expected) {
  assert.deepEqual(observe(source), expected, 'independent original JavaScript');
  const checked = checkProgram(parseJavaScript(source));
  const packet = encodeTypedProgramLinks(checked);
  const javascript = emitJavaScript(checked);
  const rust = emitRust(checked);
  const variants = [checked,
    decodeTypedProgramLinks(LinkNetwork.fromLino(packet.network.toLino()), packet.root),
    decodeTypedProgramLinks(decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network)), packet.root)];
  for (const variant of variants) {
    const program = recheckLinkedTypedProgram(variant, { wholeProgram: true });
    assert.deepEqual(emitJavaScript(program), javascript, 'complete JavaScript emission survives full rechecking');
    const restored = emitRust(program);
    for (const field of ['text', 'definitions', 'preludes', 'mappings', 'assumptions', 'encodings']) {
      assert.deepEqual(restored[field], rust[field], field + ' survives both complete archives');
    }
    assert.deepEqual(observe(emitJavaScript(program).text), expected);
  }
}

test('mutable array binding admission requires the exact dense flat literal shape', () => {
  assert.equal(acceptJavaScriptMutableArrayBinding('array', 2, 2, true, false), true);
  for (const arguments_ of [
    ['name', 2, 2, true, false], ['tuple', 2, 2, true, false],
    ['array', 0, 0, true, false], ['array', 1, 2, true, false], ['array', 3, 2, true, false],
    ['array', 2, 2, false, false], ['array', 2, 2, true, true],
    ['array', Infinity, Infinity, true, false], ['array', 1.5, 1.5, true, false],
  ]) assert.equal(acceptJavaScriptMutableArrayBinding(...arguments_), false, String(arguments_));
  for (const count of ['2', true, false, null, undefined, NaN, Infinity, -Infinity, -0, 0, 1,
    1.5, -1, Number.MAX_SAFE_INTEGER, Number.MAX_SAFE_INTEGER + 1, 2n, {}, []]) {
    const expected = Number.isSafeInteger(count) && count > 0;
    assert.equal(acceptJavaScriptMutableArrayBinding('array', count, count, true, false), expected,
      'the original safe-count predicate retains its exact Number domain');
  }
  assert.equal(acceptJavaScriptMutableArrayBindingOwner('JavaScript', false, false, false), true);
  for (const facts of [['Rust', false, false, false], ['JavaScript', true, false, false],
    ['JavaScript', false, true, false], ['JavaScript', false, false, true]]) {
    assert.equal(acceptJavaScriptMutableArrayBindingOwner(...facts), false);
  }
});

test('the shipped executor mutable Boolean declaration translates without rewriting its source', () => {
  const declaration = 'let [more, less] = [false, false];';
  const executor = readFileSync(new URL('../src/grammar-runtime/executor.js', import.meta.url), 'utf8');
  assert.equal(executor.split(declaration).length - 1, 2, 'both original precedence paths retain the exact declaration');
  const source = `function selected(first, second) {
  ${declaration}
  if (first > 0) more = true;
  if (second > 0) less = true;
  return more === less ? 0 : more ? 1 : -1;
}
console.log(selected(1,0), selected(0,1), selected(1,1), selected(0,0));`;
  equivalent(source, { lines: ['1 -1 0 0'], error: null });
  const translated = selfTranslate(source, 'JavaScript', 'Rust');
  const selected = translated.items.find(({ start }) => source.slice(start).startsWith('function selected'));
  assert.equal(selected.status, 'translated');
  assert.equal(selected.reason, null);
});

test('mutable binding lists evaluate literal elements once in order before later mutations', () => {
  equivalent(`function mark(value) { console.log(value); return value; }
function selected(seed) {
  let first = seed, [left, right] = [mark(first), mark(first + 1)], last = right;
  left += last;
  right = left + 1;
  return left + right;
}
console.log(selected(2));`, { lines: ['2', '3', '11'], error: null });
  equivalent(`function stop() { console.log('stop'); throw new Error('halt'); }
function later() { console.log('later'); return 2; }
function selected() { let [first, second] = [stop(), later()]; return first + second; }
console.log(selected());`, { lines: ['stop'], error: { name: 'Error', message: 'halt' } });
});

test('mutable literal bindings retain loop initialization block shadows and numeric values', () => {
  equivalent(`function selected() {
  let total = 0;
  for (let [index, limit] = [0, 4]; index < limit; index++) total += index;
  let [left, right] = [2, 3];
  { let [left, right] = [7, 8]; left++; console.log(left, right); }
  right += total;
  return left + right;
}
console.log(selected());`, { lines: ['8 8', '11'], error: null });
  equivalent(`function selected() { let [left, right] = [-0, Infinity]; left = 1 / left; return left + right; }
console.log(selected());`, { lines: ['NaN'], error: null });
});

test('mutable array binding temporal dead zones duplicates and unsupported receivers stay explicit', () => {
  const original = 'function f(n) { let [a] = n; return a; }';
  assert.throws(() => parseJavaScript(original), error => error.message
    === 'let destructuring: declare each binding with its own let at 16..20');
  for (const source of [
    'function selected(){let [first,second]=[second,1];return first;}',
    'function selected(){const first=9;{let [first,second]=[first,1];return second;}}',
    'function selected(){console.log(first);let [first,second]=[1,2];return second;}',
    'function selected(){let first=second,[second,last]=[1,2];return first;}',
  ]) {
    assert.equal(observe(source + 'selected();').error.name, 'ReferenceError');
    assert.throws(() => checkProgram(parseJavaScript(source)), /temporal dead zone/u);
  }
  for (const source of [
    'function selected(){let [first,first]=[1,2];return first;}',
    'function selected(){let first=1,[first]=[2];return first;}',
    'function selected(){let [first]=[1],first=2;return first;}',
  ]) {
    assert.equal(observe(source).error.name, 'SyntaxError');
    assert.throws(() => parseJavaScript(source), /(?:distinct admitted lexical identifier|duplicate lexical binding)/u);
  }
  for (const binding of [
    '[first]=values', '[first,second]=[1]', '[first]=[1,2]',
    '[first=2]=[1]', '[...first]=[1]', '[[first]]=[[1]]', '[,first]=[1,2]',
    '[first]=[...values]', '{first}={first:1}',
  ]) assert.throws(() => parseJavaScript(`function selected(values){let ${binding};return 0;}`), /let destructuring/u);
});

test('mutable literal binding owners reject actual redeclarations and preserve nested and loop shadows', () => {
  for (const source of [
    'function selected(){let left=9;let [left,right]=[1,2];return right;}',
    'function selected(){let [left,right]=[1,2];let left=9;return right;}',
    'function selected(){const left=9;let [left,right]=[1,2];return right;}',
    'function selected(){let [left,right]=[1,2];const left=9;return right;}',
    'function selected(){const [left]=[9];let [left,right]=[1,2];return right;}',
    'function selected(){let [left,right]=[1,2];const {value:left}={value:9};return right;}',
    'function selected(left){let [left,right]=[1,2];return right;}',
    'function selected(){const apply=left=>{let [left,right]=[1,2];return right;};return apply(9);}',
    'function selected(){const apply=function(left){let [left,right]=[1,2];return right;};return apply(9);}',
  ]) {
    assert.equal(observe(source).error.name, 'SyntaxError', source);
    assert.throws(() => parseJavaScript(source), /duplicate lexical binding left/u, source);
  }
  equivalent(`function selected(left) {
  for (let [left,right]=[1,2];left<right;left++) console.log(left,right);
  return left;
}
console.log(selected(9));`, { lines: ['1 2', '9'], error: null });
  equivalent(`function selected() {
  let [left,right]=[10,20];
  for(let [left,right]=[0,2];left<right;left++) {
    let [left,right]=[3,4];console.log(left,right);
  }
  return left+right;
}
console.log(selected());`, { lines: ['3 4', '3 4', '30'], error: null });
  equivalent(`function selected(){const apply=left=>{{let [left,right]=[2,3];return left+right;}};return apply(9);}
console.log(selected());`, { lines: ['5'], error: null });
  const topLevel = 'let [left,right]=[1,2];console.log(left+right);';
  assert.deepEqual(observe(topLevel), { lines: ['3'], error: null });
  assert.throws(() => parseJavaScript(topLevel), /mutable literal bindings require a function-owned lexical scope/u);
});

test('mutable literal bindings retain the standard iterator assumption and refuse its mutation', () => {
  assert.ok(JAVASCRIPT_ARRAY_BINDING_POLICY.assumptions.includes('standard array iteration and slice operations'));
  const declaration = 'function selected(){let [left,right]=[1,2];return left+right;}';
  equivalent(declaration + 'console.log(selected());', { lines: ['3'], error: null });
  const changed = declaration + 'Array.prototype[Symbol.iterator]=()=>({next:()=>({value:9,done:false})});console.log(selected());';
  assert.deepEqual(observe(changed), { lines: ['18'], error: null }, 'a fresh literal alone does not prove its active iterator');
  assert.throws(() => checkProgram(parseJavaScript(changed)), /(?:Array|Symbol|outside the portable core)/u);
});

test('mutable array decision styles borrow only their actual complete generated owner', () => {
  generateJavaScriptMutableArrayBindings({ check: true });
  const source = readFileSync(new URL('../src/translation/frontend-rules.js', import.meta.url), 'utf8');
  const emitted = emitRust(checkProgram(parseJavaScript(source)));
  const decorators = DecoratorSet.fromLino(readFileSync(new URL('../../parity/self-translation/frontend-rules-decorators.lino', import.meta.url), 'utf8'));
  const base = new DecoratorSet(decorators.decorators.filter(row => row.id !== javaScriptMutableArrayBindingStyleId));
  let raw = formatProductRuleSource(decorateEmitted('Rust', { source: emitted.text }, base).source);
  for (let round = 0; round < 10; round += 1) {
    const next = formatProductRuleSource(base.decorate('emitter', { format: 'Rust', scope: 'formatted-module', source: raw }).source);
    if (next === raw) break;
    raw = next;
    assert.ok(round < 9);
  }
  const actions = readJavaScriptMutableArrayBindingStyles(raw);
  assert.equal(actions.length, 2);
  const recorded = decorators.decorators.find(row => row.id === javaScriptMutableArrayBindingStyleId);
  assert.deepEqual(recorded.actions, actions, 'the new style is recorded from the actual whole source translation');
  const replay = new DecoratorSet([recorded]).decorate('emitter', { format: 'Rust', scope: 'formatted-module', source: raw }).source;
  const after = new Map(readRustLibraryItems(replay).items.map(item => [item.name, item]));
  for (const item of readRustLibraryItems(raw).items) {
    assert.equal(after.get(item.name).kind, item.kind);
    if (!['accept_java_script_mutable_array_binding', 'accept_java_script_mutable_array_binding_owner'].includes(item.name)) assert.equal(after.get(item.name).source, item.source);
  }
  assert.match(actions[0].to, /value_kind: &str/u);
  assert.match(actions[0].to, /#\[must_use\]/u);
  assert.doesNotMatch(actions[0].to, /#\[(?:allow|expect)\(/u);
  assert.doesNotMatch(actions[0].to, /"number"\s*==\s*"number"/u, 'the actual f64 parameter needs no native type retest');
  assert.match(actions[1].to, /language: &str/u);
  assert.match(actions[1].to, /#\[must_use\]/u);
  assert.deepEqual(readJavaScriptMutableArrayBindingStyles(replay), [], 'the whole owner style is idempotent');
  assert.throws(() => readJavaScriptMutableArrayBindingStyles('type String=Other;\n' + raw), /primitive type ownership/u);
  assert.throws(() => readJavaScriptMutableArrayBindingStyles(raw.replace('item_count: f64', 'item_count: Custom')),
    /actual primitive count parameters/u);
  assert.throws(() => readJavaScriptMutableArrayBindingStyles(raw.replace('"number" == "number"', '"number" == "string"')),
    /actual native Number type guard/u);
});
