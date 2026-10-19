import test from 'node:test';
import assert from 'node:assert/strict';
import { format } from 'node:util';
import vm from 'node:vm';
import { createTypedProgramLinks, decodeTypedProgramLinks, encodeTypedProgramLinks } from '../src/typed-program-links.js';
import { encodeLinkNetworkBinary, decodeLinkNetworkBinary } from '../src/link-network-binary.js';
import { LinkNetwork } from '../src/network.js';
import { readProgramExpressionEntryOperandField } from '../src/typed-program-link-rules.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { emitLean } from '../src/translation/emit-lean.js';
import { emitRocq } from '../src/translation/emit-rocq.js';
import { threadOutput } from '../src/translation/output.js';
import { observeThreadedOptionalProgram } from './helpers/translation-output-observer.js';

const original = 'function report(value){console.log("%s",String(value));return value;}function choose(){return {z:report(2n),a:report(1n)}.z;}console.log("%s",String(choose()));';
const reordered = 'function report(value){console.log("%s",String(value));return value;}function choose(){return {a:report(1n),z:report(2n)}.z;}console.log("%s",String(choose()));';
const observe = source => {
  const lines = [];
  vm.runInNewContext(source, { console: { log: (...values) => lines.push(format(...values)) } }, { timeout: 1000 });
  return lines;
};
const archives = packet => [LinkNetwork.fromLino(packet.network.toLino()), decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))];

function expressions(root) {
  const values = [], seen = new WeakSet();
  const visit = value => {
    if (!value || typeof value !== 'object' || seen.has(value)) return;
    seen.add(value);
    if (value.k && value.type) values.push(value);
    for (const child of Object.values(value)) visit(child);
  };
  visit(root);
  return values;
}

function moveCallsIntoActualEntries(program) {
  const fn = program.declarations.get('choose');
  const nodes = expressions(fn.body);
  const records = nodes.filter(node => node.k === 'record');
  assert.equal(records.length, 1);
  const record = records[0];
  assert.deepEqual(record.entries.map(entry => entry.name), ['a', 'z']);
  const bindings = new Map(nodes.filter(node => node.k === 'let').map(node => [node.name, node]));
  for (const entry of record.entries) {
    assert.equal(entry.value.k, 'var');
    const owner = bindings.get(entry.value.name);
    assert.equal(owner.value.k, 'call');
    assert.equal(owner.value.fn, 'report');
    const call = owner.value;
    owner.value = call.args[0];
    entry.value = call;
  }
  assert.equal(new Set(record.entries.map(entry => entry.value)).size, 2);
  return record;
}

test('record output threading uses the existing shared descriptor operand rule without changing its field labels', () => {
  assert.equal(readProgramExpressionEntryOperandField('record', 'entries'), 'value');
  assert.equal(readProgramExpressionEntryOperandField('array', 'items'), '');
  assert.deepEqual(observe(original), ['2', '1', '2']);
  const packet = createTypedProgramLinks(original, 'JavaScript');
  for (const network of archives(packet)) {
    const program = decodeTypedProgramLinks(network, packet.root);
    assert.deepEqual(observe(emitJavaScript(program).text), ['2', '1', '2']);
    assert.deepEqual(observeThreadedOptionalProgram(threadOutput(program)), { lines: ['2', '1', '2'], error: '' });
    for (const emit of [emitJavaScript, emitRust, emitLean, emitRocq]) assert.ok(emit(program).text.length > 0);
  }
});

test('source-free record entry rewires retain actual calls and evaluate each descriptor value once in its new order', () => {
  assert.deepEqual(observe(reordered), ['1', '2', '2']);
  const packet = createTypedProgramLinks(original, 'JavaScript');
  const program = decodeTypedProgramLinks(packet.network, packet.root);
  const record = moveCallsIntoActualEntries(program);
  assert.deepEqual(record.entries.map(entry => [entry.name, entry.value.fn]), [['a', 'report'], ['z', 'report']]);
  const edited = encodeTypedProgramLinks(program);
  for (const network of archives(edited)) {
    const reloaded = decodeTypedProgramLinks(network, edited.root);
    const actual = expressions(reloaded.declarations.get('choose').body).find(node => node.k === 'record');
    assert.deepEqual(actual.entries.map(entry => [entry.name, entry.value.k, entry.value.fn]), [['a', 'call', 'report'], ['z', 'call', 'report']]);
    assert.deepEqual(observe(emitJavaScript(reloaded).text), ['1', '2', '2']);
    const threaded = threadOutput(reloaded);
    assert.equal(threaded.outputThreaded, true);
    assert.deepEqual(observeThreadedOptionalProgram(threaded), { lines: ['1', '2', '2'], error: '' });
    for (const emit of [emitJavaScript, emitRust, emitLean, emitRocq]) assert.ok(emit(reloaded).text.length > 0);
  }
});

test('source-free descriptor values keep mandatory actual call-target validation', () => {
  const packet = createTypedProgramLinks(original, 'JavaScript');
  const program = decodeTypedProgramLinks(packet.network, packet.root);
  const record = moveCallsIntoActualEntries(program);
  record.entries[0].value.fn = 'absent';
  assert.throws(() => encodeTypedProgramLinks(program), error => error.message === 'the linked declaration reference absent is unresolved');
});
