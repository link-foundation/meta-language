import assert from 'node:assert/strict';
import {test} from 'node:test';
import {runInNewContext} from 'node:vm';
import {LinkNetwork} from '../src/network.js';
import {createTypedProgramLinks, decodeTypedProgramLinks} from '../src/typed-program-links.js';
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
