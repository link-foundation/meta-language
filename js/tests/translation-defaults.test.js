import assert from 'node:assert/strict';
import { test } from 'node:test';

import { translateProgram } from '../src/program-translation.js';

const DEFAULTS = `function label(name, level = 'grammar-rule', depth = 2) {
  return \`\${name}:\${level}:\${depth}\`;
}
/**
 * @param {string} term
 * @param {number[]} children
 * @returns {number}
 */
function size(term, children = []) {
  return children.length + (term === '' ? 0 : 1);
}
console.log('%s', label('a'), label('b', 'x'), label('c', 'y', 3), size('ab'), size('', [1, 2]));
`;

const refusal = (source) => translateProgram(source, 'JavaScript', 'Rust').diagnostic?.message;

test('a call that leaves out defaulted parameters passes the defaults in every target', () => {
  for (const target of ['Rust', 'Lean', 'Rocq']) {
    const translated = translateProgram(DEFAULTS, 'JavaScript', target);
    assert.equal(translated.diagnostic, null, target);
    assert.equal(translated.contract.support, 'semantic-translation', target);
  }
  const rust = translateProgram(DEFAULTS, 'JavaScript', 'Rust').code;
  assert.ok(rust.includes('pub fn label(name: String, level: String, depth: f64) -> String {'), rust);
  assert.ok(rust.includes('crate::label(String::from("a"), String::from("grammar-rule"), 2f64)'), rust);
  assert.ok(rust.includes('crate::label(String::from("b"), String::from("x"), 2f64)'), rust);
  assert.ok(rust.includes('crate::size(String::from("ab"), Vec::<f64>::new())'), rust);
});

test('defaults that are not filled in by a call are refused with a reason', () => {
  assert.match(refusal('function f(a = 1, b) {\n  return b;\n}\n'), /parameter b after a default/u);
  assert.match(refusal('function f(a, b = a) {\n  return b;\n}\nconsole.log(f(1));\n'), /default reading parameter a/u);
});

test('self-translation writes top-level constants as Rust constants and restores them', async () => {
  const { selfTranslate } = await import('../src/self-translation.js');
  const source = "const MAX_LENGTH = 2048;\nexport const TERM = 'identifier:trigram';\nexport const PAIRS = Object.freeze([['en', 'Hawaii']]);\n";
  const rust = selfTranslate(source, 'JavaScript', 'Rust');
  assert.deepEqual(rust.items.map(({ status }) => status), ['translated', 'translated', 'translated']);
  assert.ok(rust.code.includes('pub const MAX_LENGTH: f64 = 2048f64;'), rust.code);
  assert.ok(rust.code.includes('pub const TERM: &str = "identifier:trigram";'), rust.code);
  assert.ok(rust.code.includes('pub static PAIRS: std::sync::LazyLock<Vec<Vec<String>>> = std::sync::LazyLock::new(|| vec![vec![String::from("en"), String::from("Hawaii")]]);'), rust.code);
  assert.equal(selfTranslate(rust.code, 'Rust', 'JavaScript').code, source);
  // A top-level let and a statement with effects still run as a program.
  assert.equal(selfTranslate('let count = 1;\n', 'JavaScript', 'Rust').items[0].status, 'carried');
});
