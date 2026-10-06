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

const CASES = `/**
 * @param {string} name
 * @returns {string}
 */
function shout(name) {
  return name.toUpperCase() + '!' + name.toLowerCase();
}
console.log('%s', shout('Grüße ΣΑΣ'));
`;

test('toLowerCase and toUpperCase are a semantic translation into Rust, which maps Unicode case the same way', () => {
  const rust = translateProgram(CASES, 'JavaScript', 'Rust');
  assert.equal(rust.diagnostic, null);
  assert.ok(rust.code.includes('.to_uppercase()'), rust.code);
  assert.ok(rust.code.includes('.to_lowercase()'), rust.code);
  const back = translateProgram('pub fn shout(name: String) -> String {\n    name.to_lowercase().to_uppercase()\n}\n\nfn main() {\n    println!(\"{}\", shout(String::from(\"Ab\")));\n}\n', 'Rust', 'JavaScript');
  assert.equal(back.diagnostic, null);
  assert.ok(back.code.includes('.toUpperCase()'), back.code);
});

test('case mappings are refused for Lean and Rocq, whose library maps ASCII letters only', () => {
  for (const target of ['Lean', 'Rocq']) {
    assert.match(translateProgram(CASES, 'JavaScript', target).diagnostic?.message ?? '', /ASCII letters only/u, target);
  }
  assert.match(refusal("console.log('%s', 'a'.toLowerCase(1));\n"), /no arguments/u);
});
