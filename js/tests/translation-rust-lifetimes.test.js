import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import { readProjectData } from '../scripts/project-data.mjs';
import { rustFunctionLifetimeFixture } from '../scripts/generate-rust-function-lifetime-fixtures.mjs';
import { parseRust } from '../src/translation/rust.js';
import { checkProgram } from '../src/translation/check.js';
import { LinkNetwork } from '../src/network.js';
import { createTypedProgramLinks, emitTypedProgramLinks, encodeTypedProgramLinks } from '../src/typed-program-links.js';
import { recheckLinkedTypedProgram } from '../src/typed-program-checked-projection.js';
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


test('whole-program rechecking preserves ordinary qualified lifetime functions', () => {
  for (const entry of fixture.cases) {
    const program = checkProgram(parseRust(entry.source));
    assert.equal(recheckLinkedTypedProgram(program, { wholeProgram: true }), program, entry.id);
  }
});

test('ordinary lifetime module projection retains actual module and call ownership', () => {
  const source = fixture.cases.find(entry => entry.id === 'module-local-lifetime-scope').source;
  const changes = [
    ['module spelling', program => { program.items[0].fullName = 'foreign'; }],
    ['declaration module', program => { program.declarations.get('values.read').modulePath = []; }],
    ['declaration spelling', program => { program.declarations.get('values.read').fullName = 'foreign.read'; }],
    ['duplicate item', program => { program.items[0].items.push(program.items[0].items[0]); }],
    ['unowned call', program => { program.main.effects[1].expr.arg.fn = 'foreign.read'; }],
    ['missing declaration', program => { program.declarations.delete('values.read'); }],
  ];
  for (const [name, change] of changes) {
    const program = checkProgram(parseRust(source));
    change(program);
    assert.throws(() => recheckLinkedTypedProgram(program, { wholeProgram: true }), undefined, name);
    assert.throws(() => encodeTypedProgramLinks(program), undefined, name);
  }
});
