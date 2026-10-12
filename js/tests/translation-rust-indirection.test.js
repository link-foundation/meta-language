import assert from 'node:assert/strict';
import { readProjectData } from '../scripts/project-data.mjs';
import test from 'node:test';
import vm from 'node:vm';
import { rustSourceIndirectionFixture } from '../scripts/generate-rust-source-indirection-fixtures.mjs';
import { parseRust } from '../src/translation/rust.js';
import { checkProgram } from '../src/translation/check.js';
import { acceptRustImmutableIndirectionLayer, acceptRustSourceBoxOwner, acceptRustSourceIndirectionArgument, acceptRustSourceBoxCoercionLayer, acceptRustSourceBoxAvailability } from '../src/translation/rust-indirection-rules.js';

const fixture = readProjectData(new URL('../../parity/fixtures/rust-source-indirection.json', import.meta.url));

test('source Box and reference proofs preserve recursive values, forward owners and lexical scopes', () => {
  assert.deepEqual(rustSourceIndirectionFixture(), fixture);
  for (const entry of fixture.cases) {
    const output = [];
    vm.runInNewContext(entry.javascript, { console: { log: (...values) => output.push(values.map(String).join(' ')) }, process: { argv: [] } }, { timeout: 1000 });
    assert.deepEqual(output, entry.output, entry.id);
    assert.deepEqual(JSON.parse(JSON.stringify(parseRust(entry.source))), entry.surface, entry.id);
  }
  const recursive = fixture.cases.find(entry => entry.id === 'owned-recursive-box');
  assert.match(recursive.source, /sum\(\*left\).*sum\(\*right\)/u);
  assert.match(recursive.rust, /Branch\(Box<crate::Tree>, Box<crate::Tree>\)/u);
  assert.match(recursive.rust, /let left = \*left;/u);
  assert.match(recursive.rust, /let right = \*right;/u);
});

test('unproven, scalar, mutable, shadowed and leaking source pointer origins refuse precisely', () => {
  for (const entry of fixture.invalid) {
    assert.throws(() => checkProgram(parseRust(entry.source)), error => {
      assert.equal(error.kind, entry.kind, entry.id);
      assert.equal(error.reason, entry.reason, entry.id);
      assert.deepEqual(error.span ?? null, entry.span, entry.id);
      if (entry.span) assert.ok(error.span.end > error.span.start, entry.id);
      return true;
    });
  }
});

test('shared indirection decisions distinguish standard ownership and borrowed Box coercion', () => {
  for (const layer of ['box', 'reference']) assert.equal(acceptRustImmutableIndirectionLayer(layer), true);
  for (const layer of ['', 'pointer', 'mutable', 'custom']) assert.equal(acceptRustImmutableIndirectionLayer(layer), false);
  for (const owner of ['Box', 'std::boxed::Box', 'alloc::boxed::Box']) {
    assert.equal(acceptRustSourceBoxOwner(owner, false), true);
    assert.equal(acceptRustSourceBoxOwner(owner, true), false);
  }
  assert.equal(acceptRustSourceBoxOwner('Other::Box', false), false);
  assert.equal(acceptRustSourceBoxAvailability('alloc::boxed::Box', false), false);
  assert.equal(acceptRustSourceBoxAvailability('alloc::boxed::Box', true), true);
  for (const owner of ['Box', 'std::boxed::Box']) assert.equal(acceptRustSourceBoxAvailability(owner, false), true);
  assert.equal(acceptRustSourceIndirectionArgument(true, false, false, true), false);
  assert.equal(acceptRustSourceIndirectionArgument(true, false, true, true), true);
  assert.equal(acceptRustSourceIndirectionArgument(true, true, false, false), true);
  assert.equal(acceptRustSourceBoxCoercionLayer('box', true), true);
  assert.equal(acceptRustSourceBoxCoercionLayer('box', false), false);
  assert.equal(acceptRustSourceBoxCoercionLayer('reference', true), false);
});
