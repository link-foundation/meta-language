import assert from 'node:assert/strict';
import { test } from 'node:test';

import { translateProgram } from '../src/program-translation.js';

const STRINGS = `/**
 * @param {string} name
 * @returns {boolean}
 */
function isTest(name) {
  return name.endsWith('.test.js') && name.startsWith('tests/') && name.includes('/');
}
console.log(isTest('tests/a.test.js'), 'abc'.includes(''), 'abc'.startsWith('ab'), 'abc'.endsWith('b'));
`;

const refusal = (source) => translateProgram(source, 'JavaScript', 'Rust').diagnostic?.message;

test('startsWith, endsWith and includes on strings are a semantic translation in every target', () => {
  for (const target of ['Rust', 'Lean', 'Rocq']) {
    const translated = translateProgram(STRINGS, 'JavaScript', target);
    assert.equal(translated.diagnostic, null, target);
    assert.equal(translated.contract.support, 'semantic-translation', target);
  }
  const rust = translateProgram(STRINGS, 'JavaScript', 'Rust').code;
  assert.ok(rust.includes('.ends_with('), rust);
  assert.ok(rust.includes('.starts_with('), rust);
  assert.ok(rust.includes('.contains('), rust);
  const lean = translateProgram(STRINGS, 'JavaScript', 'Lean').code;
  assert.ok(lean.includes('def ml_string_includes'), lean);
  const rocq = translateProgram(STRINGS, 'JavaScript', 'Rocq').code;
  assert.ok(rocq.includes('String.prefix'), rocq);
});

test('string tests that are not portable are refused with a reason', () => {
  assert.match(refusal("console.log('abc'.startsWith('a', 1));\n"), /one argument/u);
  assert.match(refusal("const xs = [1];\nconsole.log(xs.includes(1));\n"), /portable on strings/u);
});
