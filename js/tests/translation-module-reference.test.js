import assert from 'node:assert/strict';
import { test } from 'node:test';
import { selfTranslate, selfTranslationSignatures } from '../src/self-translation.js';

const identity = `/** @param {number[]} values @returns {number[]} */
export function identity(values) { return values; }`;

function assertReferenceSignature(signature) {
  assert.equal(signature.params[0].type.reference, true);
  assert.equal(signature.ret.reference, true);
}

test('a forced reference representation survives signature discovery and module emission', () => {
  const options = { referenceSemantics: true };
  const signatures = selfTranslationSignatures(identity, 'JavaScript', options);
  assertReferenceSignature(signatures[0]);
  const translated = selfTranslate(identity, 'JavaScript', 'Rust', options);
  assert.ok(translated.items.every((item) => item.status === 'translated'));
  assert.match(translated.code, /pub fn identity\(values: std::rc::Rc<std::cell::RefCell<Vec<f64>>>\) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>>/u);
});

test('a mutable closure selects compatible reference signatures for sibling declarations', () => {
  const source = `${identity}\nexport function counter() {
    let count = 0;
    const next = () => { count = count + 1; return count; };
    return next();
  }`;
  const signatures = selfTranslationSignatures(source, 'JavaScript');
  assertReferenceSignature(signatures.find((signature) => signature.name === 'identity'));
  const translated = selfTranslate(source, 'JavaScript', 'Rust');
  assert.ok(translated.items.every((item) => item.status === 'translated'), JSON.stringify(translated.items));
  assert.match(translated.code, /std::rc::Rc::new\(std::cell::RefCell::new\(0f64\)\)/u);
});

test('imported reference signatures select the same representation in the consumer module', () => {
  const imports = { './identity.js': selfTranslationSignatures(identity, 'JavaScript', { referenceSemantics: true }) };
  const source = `import { identity } from './identity.js';
/** @param {number[]} values @returns {number[]} */
export function forward(values) { return identity(values); }`;
  const signatures = selfTranslationSignatures(source, 'JavaScript', { imports });
  assertReferenceSignature(signatures.find((signature) => signature.name === 'forward'));
  const translated = selfTranslate(source, 'JavaScript', 'Rust', { imports });
  assert.ok(translated.items.every((item) => item.status === 'translated'), JSON.stringify(translated.items));
  assert.match(translated.code, /use crate::identity::identity;/u);
});
