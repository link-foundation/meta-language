import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import vm from 'node:vm';

import {
  readRequiredBooleanFact, readRequiredNumberFact, readRequiredTextFact,
} from '../src/translation/fact-column-rules.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { parseJavaScript } from '../src/translation/javascript.js';

const source = readFileSync(new URL('../src/translation/fact-column-rules.js', import.meta.url), 'utf8');

test('required fact columns preserve every present primitive value and its selected position', () => {
  const numbers = [0, -0, 2.5, -7, Number.NaN, Infinity];
  for (let at = 0; at < numbers.length; at += 1) assert.ok(Object.is(readRequiredNumberFact(numbers, at), numbers[at]));
  assert.ok(Object.is(readRequiredNumberFact(numbers, -0), 0));
  for (const [values, read] of [[['', 'x', '😀', '\ud800'], readRequiredTextFact], [[false, true], readRequiredBooleanFact]]) {
    values.forEach((value, at) => assert.equal(read(values, at), value));
  }
});

test('required fact columns refuse actual missing or invalid positions without substituting values', () => {
  for (const read of [readRequiredNumberFact, readRequiredTextFact, readRequiredBooleanFact]) {
    for (const index of [-1, 1, 0.5, Number.NaN, Infinity, -Infinity]) assert.throws(() => read([1], index), RangeError);
    assert.throws(() => read([], 0), RangeError);
    assert.throws(() => read(Array(1), 0), RangeError);
    assert.throws(() => read([undefined], 0), RangeError);
  }
});

test('the complete required-column source self-translates with real optional reads and abort branches', () => {
  const checked = checkProgram(parseJavaScript(source));
  const javascript = emitJavaScript(checked).text;
  assert.equal(vm.runInNewContext(javascript + '\nreadRequiredNumberFact([8, 13], 1);', {}, { timeout: 1000 }), 13);
  assert.equal(vm.runInNewContext(javascript + '\nreadRequiredTextFact(["", "x"], 0);', {}, { timeout: 1000 }), '');
  assert.equal(vm.runInNewContext(javascript + '\nreadRequiredBooleanFact([false], 0);', {}, { timeout: 1000 }), false);
  assert.throws(() => vm.runInNewContext(javascript + '\nreadRequiredNumberFact([8], 1);', {}, { timeout: 1000 }), /invalid required Number fact column position/u);
  const rust = emitRust(checked);
  assert.equal(rust.definitions.length, 6, 'all three read functions and all three genuine abort functions are translated');
  assert.match(rust.text, /optional_at/u);
  assert.match(rust.text, /unwrap_or_else/u);
  assert.match(rust.text, /panic!/u);
  assert.equal(rust.assumptions?.some(assumption => /array reads are in bounds/u.test(assumption)), false);
});
