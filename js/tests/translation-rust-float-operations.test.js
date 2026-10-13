import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import { readProjectData } from '../scripts/project-data.mjs';
import { rustFloatOperationFixture } from '../scripts/generate-rust-float-operation-fixtures.mjs';
import { parseRust } from '../src/translation/rust.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { LinkNetwork } from '../src/network.js';
import { createTypedProgramLinks, decodeTypedProgramLinks, emitTypedProgramLinks } from '../src/typed-program-links.js';
import { acceptRustFloatOperationMethod } from '../src/translation/rust-float-predicate-rules.js';
import { createRustFloatOperationExpression, RUST_FLOAT_PREDICATE_POLICY } from '../src/translation/rust-float-predicates.js';

const fixture = readProjectData(new URL('../../parity/fixtures/rust-float-operations.lino', import.meta.url));
function tag(value) {
  if (typeof value === 'boolean') return String(value);
  assert.equal(typeof value, 'number');
  if (Number.isNaN(value)) return 'NaN';
  const view = new DataView(new ArrayBuffer(8));
  view.setFloat64(0, value, false);
  return view.getBigUint64(0, false).toString(16).padStart(16, '0');
}
function observed(source, observations, extra = {}) {
  const context = { ...extra };
  vm.runInNewContext(source, context, { timeout: 1000 });
  return observations.map(row => tag(context.apply(...row.javascriptArguments.map(value => Array.isArray(value) ? Object.freeze(value.map(Number)) : Number(value)))));
}
function nodes(value, predicate, result = []) {
  if (!value || typeof value !== 'object') return result;
  if (predicate(value)) result.push(value);
  for (const child of Object.values(value)) nodes(child, predicate, result);
  return result;
}
test('primitive f64 operations preserve independent bit oracles and source-free replay', () => {
  assert.deepEqual(rustFloatOperationFixture(), fixture);
  for (const entry of fixture.cases) {
    const expected = entry.observations.map(row => row.expectedBits);
    assert.deepEqual(observed(entry.originalJavaScript, entry.observations), expected, entry.id);
    const original = checkProgram(parseJavaScript(entry.originalJavaScript));
    assert.deepEqual(observed(emitJavaScript(original).text, entry.observations), expected, entry.id);
    assert.deepEqual(JSON.parse(JSON.stringify(parseRust(entry.source))), entry.surface, entry.id);
    assert.deepEqual(observed(entry.javascript, entry.observations), expected, entry.id);
    const archive = createTypedProgramLinks(entry.source, 'Rust');
    const network = LinkNetwork.fromLino(archive.network.toLino());
    assert.ok(network.links().every(link => !link.metadata().definition?.includes(entry.source)), entry.id);
    const javascript = emitTypedProgramLinks(network, archive.root, 'JavaScript').code;
    assert.equal(javascript, entry.javascript, entry.id);
    assert.deepEqual(observed(javascript, entry.observations), expected, entry.id);
    assert.ok(emitTypedProgramLinks(network, archive.root, 'Rust').code.includes(entry.rustDefinitions), entry.id);
  }
});
test('all actual operation definitions reimport through Rust and JavaScript frontends', () => {
  for (const entry of fixture.cases) {
    const expected = entry.observations.map(row => row.expectedBits);
    const replay = checkProgram(parseRust(entry.rustDefinitions + '\nfn main(){}'));
    assert.equal(emitRust(replay).definitions.join('\n\n'), entry.rustDefinitions, entry.id);
    assert.deepEqual(observed(emitJavaScript(replay).text, entry.observations), expected, entry.id);
    const reverse = checkProgram(parseJavaScript(entry.javascriptDefinitions));
    const returnedDefinitions = emitRust(reverse).definitions.join('\n\n');
    assert.equal(returnedDefinitions, entry.rustDefinitions, entry.id);
    const returned = checkProgram(parseRust(returnedDefinitions + '\nfn main(){}'));
    assert.deepEqual(observed(emitJavaScript(returned).text, entry.observations), expected, entry.id);
    assert.deepEqual(observed(entry.reverseReplayJavaScript, entry.observations), expected, entry.id);
  }
  const zero = fixture.cases.find(entry => entry.id === 'typed-negative-zero-binding');
  assert.ok(zero.rustDefinitions.includes('let value = (-0f64);'));
  assert.equal(tag(-0), '8000000000000000');
});
test('each shared operation template evaluates its actual function receiver once', () => {
  for (const [method, template] of Object.entries(RUST_FLOAT_PREDICATE_POLICY.operationTemplates)) {
    assert.equal(nodes(template, node => node.slot === 'receiver').length, 1, method);
    const receiver = { k: 'name', path: ['provider'], span: { start: 2, end: 10 } };
    const result = createRustFloatOperationExpression(method, receiver, { start: 2, end: 18 });
    assert.equal(nodes(result, node => node === receiver).length, 1, method);
    assert.deepEqual(result.span, { start: 2, end: 18 });
    assert.deepEqual(receiver.span, { start: 2, end: 10 });
    const entry = fixture.cases.find(candidate => candidate.id === `receiver-call-once-${method}`);
    const archive = createTypedProgramLinks(entry.source, 'Rust');
    const network = LinkNetwork.fromLino(archive.network.toLino());
    const program = decodeTypedProgramLinks(network, archive.root);
    assert.equal(nodes(program.items.find(item => item.name === 'apply').body, node => node.k === 'call' && node.fn === 'provider').length, 1, method);
    const code = emitTypedProgramLinks(network, archive.root, 'JavaScript').code;
    const state = { calls: 0 };
    const instrumented = code.replace('function provider() {', 'function provider() { globalThis.counter.calls += 1;');
    assert.notEqual(instrumented, code, method);
    assert.deepEqual(observed(instrumented, entry.observations, { counter: state }), entry.observations.map(row => row.expectedBits), method);
    assert.equal(state.calls, entry.receiverCalls, method);
  }
});
test('wrong arity, custom, shadowed, f32, indirected and unproved receivers refuse precisely', () => {
  for (const entry of fixture.invalid) assert.throws(() => checkProgram(parseRust(entry.source)), error => {
    assert.equal(error.kind, entry.kind, entry.id);
    assert.equal(error.reason, entry.reason, entry.id);
    assert.deepEqual(error.span ?? null, entry.span, entry.id);
    return true;
  });
  for (const method of ['abs', 'floor', 'ceil', 'trunc']) assert.equal(acceptRustFloatOperationMethod(method), true);
  for (const method of ['round', 'signum', 'sin', 'cos', 'sqrt', 'is_nan']) assert.equal(acceptRustFloatOperationMethod(method), false);
});
test('source-free operation edits preserve checked types and alter exact value observations', () => {
  const entry = fixture.cases.find(candidate => candidate.id === fixture.edit.case);
  const archive = createTypedProgramLinks(entry.source, 'Rust');
  const network = LinkNetwork.fromLino(archive.network.toLino());
  const operation = network.fieldRelations().find(relation => relation.label === 'operation' && network.link(relation.child).metadata().definition === fixture.edit.from);
  assert.ok(operation);
  const scalar = network.link(operation.child);
  scalar.setMetadata(scalar.metadata().withDefinition(fixture.edit.to));
  const changed = LinkNetwork.fromLino(network.toLino());
  const emitted = emitTypedProgramLinks(changed, archive.root, 'JavaScript').code;
  assert.deepEqual(observed(emitted, [{ javascriptArguments: [fixture.edit.input] }]), [fixture.edit.expectedBits]);
  assert.match(emitTypedProgramLinks(changed, archive.root, 'Rust').code, /\.ceil\(\)/u);
  assert.doesNotThrow(() => decodeTypedProgramLinks(changed, archive.root));
});
