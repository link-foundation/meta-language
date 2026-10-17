import assert from 'node:assert/strict';
import {test} from 'node:test';
import {runInNewContext} from 'node:vm';
import {format} from 'node:util';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary, decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {createTypedProgramLinks, decodeTypedProgramLinks, TYPED_PROGRAM_LINKS_SCHEMA} from '../src/typed-program-links.js';
import {recheckLinkedTypedProgram} from '../src/typed-program-checked-projection.js';
import {projectLinkedProgramExpression} from '../src/typed-program-expression-projection.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';

test('source-free method projection retains the actual omitted separator slot and whole-program behavior', () => {
  const source = '/** @param {string} value @returns {string[]} */ function split(value){return value.split();}';
  const packet = createTypedProgramLinks(source, 'JavaScript');
  const restored = decodeTypedProgramLinks(LinkNetwork.fromLino(packet.network.toLino()), packet.root);
  const body = restored.declarations.get('split').body;
  assert.equal(body.k, 'method');
  assert.equal(body.args.length, 2);
  assert.equal(body.args[0], null);
  assert.equal(body.args[1].value, '4294967295');
  const surface = projectLinkedProgramExpression(body);
  assert.equal(surface.args.length, 2);
  assert.equal(surface.args[0], null);
  assert.strictEqual(recheckLinkedTypedProgram(restored, {wholeProgram: true}), restored);
  const context = {};
  runInNewContext(emitJavaScript(restored).text, context, {timeout: 1000});
  assert.deepEqual(Array.from(context.split('a,b')), ['a,b']);
});

test('the added proven-index projection retains its proof while the old plan stays unqualified', () => {
  const float = {kind: 'float'};
  const base = {k: 'index', array: {k: 'var', name: 'values', type: {kind: 'array', element: float}}, index: {k: 'lit', type: float, value: '0'}, type: float};
  const plain = projectLinkedProgramExpression(base);
  assert.equal(Object.hasOwn(plain, 'provenInBounds'), false);
  const proven = projectLinkedProgramExpression({...base, provenInBounds: true});
  assert.equal(proven.k, 'index');
  assert.equal(proven.provenInBounds, true);
  assert.deepEqual(proven.object, plain.object);
  assert.deepEqual(proven.index, plain.index);
});

test('optional result rechecking retains the concrete source annotation and expanded semantic result', () => {
  const source = '/** @param {number[]} values @returns {number} */ function first(values){return values[0];}';
  const packet = createTypedProgramLinks(source, 'JavaScript');
  const restored = decodeTypedProgramLinks(LinkNetwork.fromLino(packet.network.toLino()), packet.root);
  const entry = restored.declarations.get('first');
  assert.equal(entry.annotatedRet.kind, 'float');
  assert.equal(entry.ret.kind, 'option');
  assert.equal(entry.ret.element.kind, 'float');
  assert.strictEqual(recheckLinkedTypedProgram(restored, {wholeProgram: true}), restored);
  const context = {};
  runInNewContext(emitJavaScript(restored).text, context, {timeout: 1000});
  assert.equal(context.first([]), undefined);
  assert.equal(context.first([9]), 9);
});

function readOptionalTextNodes(program) {
  const found = [], visited = new WeakSet();
  function visit(value) {
    if (!value || typeof value !== 'object' || visited.has(value)) return;
    visited.add(value);
    if (value.k === 'optionalText') found.push(value);
    for (const child of value instanceof Map ? value.values() : Object.values(value)) visit(child);
  }
  visit(program);return found;
}

const optionalTextSource = '/** @param {bigint} [value] @returns {bigint} */\nfunction identity(value){return value;}\n'
  + 'console.log(identity(7n));console.log(String(identity(7n)));console.log(identity());console.log(String(identity()));\n';

test('source-free optional text projections retain distinct String and console observations through both archives', () => {
  const plans = TYPED_PROGRAM_LINKS_SCHEMA.expressionProjections;
  assert.deepEqual(plans.optionalText, {variant: ['console']});
  assert.deepEqual(plans['optionalText:false'], {construct: 'toString', fields: [['arg', 'expression', 'arg']]});
  assert.deepEqual(plans['optionalText:true'], {construct: 'show', fields: [['arg', 'expression', 'arg'], ['style', 'constant', 'js-console']]});
  const observe = source => {
    const lines = [];runInNewContext(source, {console: {log: (...values) => lines.push(format(...values))}}, {timeout: 1000});return lines;
  };
  const expected = ['7n', '7', 'undefined', 'undefined'];
  assert.deepEqual(observe(optionalTextSource), expected);
  const packet = createTypedProgramLinks(optionalTextSource, 'JavaScript');
  for (const [encoding, network] of [
    ['text', LinkNetwork.fromLino(packet.network.toLino())],
    ['binary', decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))],
  ]) {
    const restored = decodeTypedProgramLinks(network, packet.root), nodes = readOptionalTextNodes(restored);
    assert.equal(nodes.length, 4, encoding + ': every actual optional conversion is retained');
    assert.deepEqual(nodes.map(node => node.console).sort(), [false, false, true, true]);
    for (const node of nodes) {
      const surface = projectLinkedProgramExpression(node, restored.declarations);
      assert.equal(surface.k, node.console ? 'show' : 'toString');
      if (node.console) assert.equal(surface.style, 'js-console');
      else assert.equal(Object.hasOwn(surface, 'style'), false);
    }
    assert.strictEqual(recheckLinkedTypedProgram(restored, {wholeProgram: true}), restored);
    assert.deepEqual(observe(emitJavaScript(restored).text), expected, encoding + ': complete rechecked program behavior');
  }
});

test('optional text whole-program rechecking rejects missing malformed and coerced variants and forged types', () => {
  const packet = createTypedProgramLinks(optionalTextSource, 'JavaScript');
  const original = decodeTypedProgramLinks(LinkNetwork.fromLino(packet.network.toLino()), packet.root);
  for (const invalid of [undefined, null, 0, 1, '', 'false', 'true', [], {}]) {
    const forged = structuredClone(original);readOptionalTextNodes(forged)[0].console = invalid;
    assert.throws(() => recheckLinkedTypedProgram(forged, {wholeProgram: true}), TypeError, 'the final typed boundary cannot coerce a discriminator: ' + String(invalid));
  }
  for (const mutate of [
    node => {delete node.console;}, node => {delete node.arg;}, node => {node.type = {kind: 'bool'};},
    node => {node.arg.type = {kind: 'bool'};}, node => {node.unrecognizedProjectionField = true;},
  ]) {
    const forged = structuredClone(original);mutate(readOptionalTextNodes(forged)[0]);
    assert.throws(() => recheckLinkedTypedProgram(forged, {wholeProgram: true}), TypeError);
  }
});
