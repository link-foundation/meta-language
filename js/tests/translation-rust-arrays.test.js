import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import { readProjectData } from '../scripts/project-data.mjs';
import { rustArraySourceFixture } from '../scripts/generate-rust-array-source-fixtures.mjs';
import { parseRust } from '../src/translation/rust.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { LinkNetwork } from '../src/network.js';
import { createTypedProgramLinks, emitTypedProgramLinks } from '../src/typed-program-links.js';
import { acceptRustSourceArrayOwner, acceptRustSourceArrayElement, acceptRustSourceArrayCoercion } from '../src/translation/rust-array-rules.js';
import {assertRustFreshVectorResultCorrection} from './support/rust-fresh-vector-result-corrections.js';

const fixture = readProjectData(new URL('../../parity/fixtures/rust-array-source.lino', import.meta.url));
function observed(source, name, inputs) {
  const context = {};
  vm.runInNewContext(source, context, { timeout: 1000 });
  return inputs.map(input => context[name](Object.freeze([...input])));
}
test('selected array policies reparse actual emitted Rust and reload through source-free Links', () => {
  assert.deepEqual(rustArraySourceFixture(), fixture);
  for (const entry of fixture.cases) {
    assert.deepEqual(JSON.parse(JSON.stringify(parseRust(entry.source))), entry.surface, entry.id);
    assert.deepEqual(observed(entry.javascript, entry.name, entry.inputs), entry.expected, entry.id);
    assert.deepEqual(observed(entry.replayJavaScript, entry.name, entry.inputs), entry.expected, entry.id);
    const archive = createTypedProgramLinks(entry.source, 'Rust');
    const network = LinkNetwork.fromLino(archive.network.toLino());
    assert.deepEqual(observed(emitTypedProgramLinks(network, archive.root, 'JavaScript').code, entry.name, entry.inputs), entry.expected, entry.id);
    const emitted = emitTypedProgramLinks(network, archive.root, 'Rust').code;
    assert.ok(emitted.includes(entry.rustDefinitions), entry.id);
    const checkedReplay = checkProgram(parseRust(entry.rustDefinitions + '\nfn main(){}'));
    assert.equal(emitRust(checkedReplay).definitions.join('\n\n'), entry.rustDefinitions, entry.id);
    assert.deepEqual(observed(emitJavaScript(checkedReplay).text, entry.name, entry.inputs), entry.expected, entry.id);
  }
});
test('array source ownership, unsupported borrowing and generic boundaries retain precise refusals', () => {
  for (const entry of fixture.invalid) {
   if(['array-return','unconverted-source-length','other-length-conversion','source-indexing'].includes(entry.id))assertRustFreshVectorResultCorrection(entry);
   else assert.throws(() => checkProgram(parseRust(entry.source)), error => {
    assert.equal(error.kind, entry.kind, entry.id);
    assert.equal(error.reason, entry.reason, entry.id);
    assert.deepEqual(error.span ?? null, entry.span, entry.id);
    return true;
   });
  }
});
test('standard vector construction and borrowing preserve read-only observations', () => {
  for (const initializer of ['vec![1f64,2f64]', 'Vec::<f64>::new()']) {
    const source = `fn empty(values:&[f64])->bool{values.is_empty()} fn main(){let values:Vec<f64> = ${initializer};println!("{}",empty(&values));}`;
    const output = [];
    vm.runInNewContext(emitJavaScript(checkProgram(parseRust(source))).text, { console: { log: value => output.push(value) } });
    assert.deepEqual(output, [initializer.includes('new') ? 'true' : 'false']);
  }
});
test('shared array decisions retain standard owners, direct elements and one-way slice coercion', () => {
  for (const owner of ['Vec', 'std::vec::Vec']) {
    assert.equal(acceptRustSourceArrayOwner(owner, false), true);
    assert.equal(acceptRustSourceArrayOwner(owner, true), false);
  }
  for (const owner of ['Other::Vec', 'alloc::vec::Vec']) assert.equal(acceptRustSourceArrayOwner(owner, false), false);
  for (const kind of ['bool', 'float', 'fixed', 'unit', 'string']) {
    assert.equal(acceptRustSourceArrayElement(kind, false), true);
    assert.equal(acceptRustSourceArrayElement(kind, true), false);
  }
  for (const kind of ['array', 'record', 'named', 'function', 'option']) assert.equal(acceptRustSourceArrayElement(kind, false), false);
  assert.equal(acceptRustSourceArrayCoercion('owned vector', 'immutable slice', true), true);
  assert.equal(acceptRustSourceArrayCoercion('immutable slice', 'owned vector', true), false);
  assert.equal(acceptRustSourceArrayCoercion('owned vector', 'immutable slice', false), false);
});
