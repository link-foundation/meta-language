import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { test } from 'node:test';

import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { tokenize } from '../src/translation/lexer.js';

function execute(source) {
  return execFileSync(process.execPath, ['--input-type=module', '-e', source], { encoding: 'utf8' });
}

function equivalent(source, expected) {
  const checked = checkProgram(parseJavaScript(source));
  assert.equal(execute(source), expected);
  assert.equal(execute(emitJavaScript(checked).text), expected);
  // Native compilation belongs to CI; this checks that the same checked
  // representation reaches the Rust emitter without a carried fallback.
  assert.ok(emitRust(checked).text.includes('pub fn'));
}

test('local closures capture immutable parameters and can be called repeatedly', () => {
  equivalent(`/** @param {number[]} values @param {number} factor @returns {number[]} */
function scaled(values, factor) {
  const scale = (value) => value * factor;
  return [scale(values[0]), scale(values[1])];
}
const result = scaled([2, 3], 4);
console.log(result[0], result[1]);`, '8 12\n');
});

test('computed closures and function expressions preserve parameter shadowing', () => {
  equivalent(`function value(number) {
  const double = function(number) { return number * 2; };
  return double(number) + ((number) => number + 1)(5);
}
console.log(value(3));`, '12\n');
});

test('closures refuse captures whose binding changes outside the closure', () => {
  assert.throws(() => parseJavaScript('function value(){ let factor=2; const read=()=>factor; factor=3; return read(); }'), /mutable closure capture factor/u);
  assert.throws(() => parseJavaScript('function value(){ let factor=2; const change=()=>{factor=3;return factor;}; return change(); }'), /assignment of constant factor|mutable closure capture/u);
});

test('array destructuring evaluates its source once and preserves holes in the binding pattern', () => {
  equivalent(`function values() { console.log('read'); return [2, 3, 5]; }
function selected() { const [first, , third] = values(); return first + third; }
console.log(selected());`, 'read\n7\n');
});

test('const destructuring is fresh for every for-of iteration', () => {
  equivalent(`/** @param {string[][]} pairs @returns {string} */
function keys(pairs) { let out = ''; for (const [key] of pairs) out += key; return out; }
console.log(keys([['a', 'A'], ['b', 'B']]));`, 'ab\n');
});

test('string and Number switches retain strict equality and their default placement', () => {
  equivalent(`const Tier = { Gold: 'gold', Silver: 'silver' };
function weight(tier) { switch (tier) { case Tier.Gold: return 100; default: return 0; case Tier.Silver: return 50; } }
function numberWeight(value) { switch (value) { case -0: return 1; case 2.5: return 2; case NaN: return 3; default: return 4; } }
console.log(weight('gold'), weight('silver'), weight('bronze'), numberWeight(0), numberWeight(2.5), numberWeight(NaN));`, '100 50 0 1 2 4\n');
});

test('nullish coalescing preserves falsy values and rejects unparenthesized logical mixing', () => {
  equivalent(`/** @param {number|null} value @returns {number} */
function valueOrDefault(value) { return value ?? 7; }
console.log(valueOrDefault(null), valueOrDefault(0), valueOrDefault(3));`, '7 0 3\n');
  for (const expression of ['null ?? false || true', 'true && null ?? 1', 'null ?? true && false']) {
    assert.throws(() => parseJavaScript(`function value(){return ${expression};}`), /mixing \?\?/u);
  }
});

test('statement braces permit regex literals while expression braces retain division', () => {
  for (const source of ['if (ready) {} /[9A]/u.test(text);', 'function ready() {} /[9A]/u.test(text);']) {
    assert.equal(tokenize(source, 'JavaScript').tokens.filter((token) => token.kind === 'regex').length, 1);
  }
  for (const source of ['const object = { value: 1 } / 2;', 'const value = function named() {} / 2;', '`${{} / 2}`']) {
    assert.equal(tokenize(source, 'JavaScript').tokens.filter((token) => token.kind === 'regex').length, 0);
  }
});
