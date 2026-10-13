import assert from 'node:assert/strict';
import { test } from 'node:test';
import { checkProgram } from '../src/translation/check.js';
import { parseRust } from '../src/translation/rust.js';

function sourceWithHeader(owner, deep = false) {
  const header = `fn header(value:${owner})->bool{true}`;
  const declaration = deep ? `mod deep{${header}}` : header;
  return `enum f64{Only}mod nested{fn f64()->bool{true}${declaration}fn value()->bool{f64()}}fn main(){}`;
}

function parseHeader(owner, deep = false) {
  const surface = parseRust(sourceWithHeader(owner, deep), { sourceSignatureModel: 'Rust source function signature 1' });
  const nested = surface.items.find(item => item.name === 'nested');
  const container = deep ? nested.items.find(item => item.name === 'deep') : nested;
  return { surface, owner: container.items.find(item => item.name === 'header').sourceSignature.parameters[0].owner };
}

test('anchored type owner lookup never falls through a value namespace hit through actual Rust source', () => {
  // The unqualified type ignores nested.f64, a real value function, and finds root data.
  assert.deepEqual(parseHeader('f64').owner, { kind: 'declaration', path: ['f64'], arguments: [] });

  const self = parseHeader('self::f64');
  assert.deepEqual(self.owner, { kind: 'declaration', path: ['self', 'f64'], arguments: [] });
  assert.throws(() => checkProgram(self.surface), error => error.reason === 'unknown type self.f64');

  const parent = parseHeader('super::f64', true);
  assert.deepEqual(parent.owner, { kind: 'declaration', path: ['super', 'f64'], arguments: [] });
  assert.throws(() => checkProgram(parent.surface), error => error.reason === 'unknown type super.f64');

  const root = parseHeader('crate::f64');
  assert.deepEqual(root.owner, { kind: 'declaration', path: ['f64'], arguments: [] });
  assert.deepEqual(checkProgram(root.surface).declarations.get('nested.header').params[0].type, { kind: 'data', name: 'f64' });

  const checked = checkProgram(parseRust(sourceWithHeader('crate::f64')));
  assert.equal(checked.declarations.get('nested.value').body.fn, 'nested.f64');
  assert.deepEqual(checked.declarations.get('nested.value').body.args, []);
});
