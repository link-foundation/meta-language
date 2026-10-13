import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import { readProjectData } from '../scripts/project-data.mjs';
import { rustFunctionLifetimeFixture } from '../scripts/generate-rust-function-lifetime-fixtures.mjs';
import { parseRust } from '../src/translation/rust.js';
import { checkProgram } from '../src/translation/check.js';
import { LinkNetwork } from '../src/network.js';
import { createTypedProgramLinks, emitTypedProgramLinks } from '../src/typed-program-links.js';
import { acceptRustSourceLifetimeBinding, acceptRustSourceLifetimeReference, acceptRustSourceLifetimeResult } from '../src/translation/rust-indirection-rules.js';

const fixture = readProjectData(new URL('../../parity/fixtures/rust-function-lifetimes.lino', import.meta.url));
function observed(source) {
  const output = [];
  vm.runInNewContext(source, { console: { log: (...values) => output.push(values.map(String).join(' ')) } }, { timeout: 1000 });
  return output;
}

test('lifetime-only headers retain immutable input value behavior through source-free Links', () => {
  assert.deepEqual(rustFunctionLifetimeFixture(), fixture);
  for (const entry of fixture.cases) {
    assert.deepEqual(JSON.parse(JSON.stringify(parseRust(entry.source))), entry.surface, entry.id);
    assert.deepEqual(observed(entry.javascript), entry.output, entry.id);
    const archive = createTypedProgramLinks(entry.source, 'Rust');
    const network = LinkNetwork.fromLino(archive.network.toLino());
    assert.deepEqual(observed(emitTypedProgramLinks(network, archive.root, 'JavaScript').code), entry.output, entry.id);
    assert.equal(emitTypedProgramLinks(network, archive.root, 'Rust').code, entry.rust, entry.id);
    assert.ok([...network.links()].every(link => !link.metadata().definition?.includes(entry.source)), entry.id);
  }
});

test('undeclared, leaking, mutable, generic and returned lifetime scopes refuse precisely', () => {
  for (const entry of fixture.invalid) {
    assert.throws(() => checkProgram(parseRust(entry.source)), error => {
      assert.equal(error.kind, entry.kind, entry.id);
      assert.equal(error.reason, entry.reason, entry.id);
      assert.deepEqual(error.span ?? null, entry.span, entry.id);
      assert.ok(entry.span.end > entry.span.start, entry.id);
      return true;
    });
  }
});

test('shared lifetime decisions distinguish declared inputs from return-lifetime obligations', () => {
  for (const name of ['a', 'value', 'outer', 'inner']) {
    assert.equal(acceptRustSourceLifetimeBinding(name, false, false), true);
    assert.equal(acceptRustSourceLifetimeBinding(name, true, false), false);
    assert.equal(acceptRustSourceLifetimeBinding(name, false, true), false);
    assert.equal(acceptRustSourceLifetimeReference(name, true), true);
    assert.equal(acceptRustSourceLifetimeReference(name, false), false);
  }
  for (const name of ['static', '_', 'fn', 'Self', 'const', 'gen']) assert.equal(acceptRustSourceLifetimeBinding(name, false, false), false);
  for (const name of ['static', '_']) assert.equal(acceptRustSourceLifetimeReference(name, false), true);
  assert.equal(acceptRustSourceLifetimeResult(true, true), false);
  for (const [named, reference] of [[false, false], [false, true], [true, false]]) assert.equal(acceptRustSourceLifetimeResult(named, reference), true);
});
