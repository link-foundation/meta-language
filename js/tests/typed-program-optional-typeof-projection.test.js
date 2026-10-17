// Bounded optional type query source-free controls. Native execution is CI-only.
import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary, decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {encodeTypedProgramLinks, decodeTypedProgramLinks} from '../src/typed-program-links.js';
import {recheckLinkedTypedProgram} from '../src/typed-program-checked-projection.js';
import {projectLinkedProgramExpression} from '../src/typed-program-expression-projection.js';

const forms = packet => [['text', LinkNetwork.fromLino(packet.network.toLino())],
  ['binary', decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]];
function observe(source) {
  const output = [], context = {console: {log: (...values) => output.push(values.map(String).join(' '))}};
  let failure = null;
  try { vm.runInNewContext(source, context, {timeout: 1000}); }
  catch (error) { failure = {name: error.name, message: error.message}; }
  return {output, failure};
}
function savedPrograms(source) {
  const checked = checkProgram(parseJavaScript(source, {referenceSemantics: false}));
  const packet = encodeTypedProgramLinks(checked);
  return [checked, ...forms(packet).map(([format, network]) => {
    const restored = decodeTypedProgramLinks(network, packet.root);
    const before = structuredClone(restored);
    const rechecked = recheckLinkedTypedProgram(restored, {wholeProgram: true});
    assert.deepEqual(restored, before, format + ': rechecking does not normalize input data');
    return rechecked;
  })];
}
const source = '/** @param {string[]} values @param {number} start @returns {string} */\n'
  + 'export function query(values,start){return typeof values[start+10];}\n'
  + 'console.log(query(["a","b","c","d","e","f","g","h","i","j","why"],0));console.log(query([],0));';

test('optional type query rederives present from its real operand through both complete archives', () => {
  const expected = {output: ['string', 'undefined'], failure: null};
  assert.deepEqual(observe(source.replace('export ', '')), expected);
  const programs = savedPrograms(source), originalRust = emitRust(programs[0]);
  for (const program of programs) {
    const query = program.declarations.get('query').body;
    assert.equal(query.k, 'optionalTypeOf'); assert.equal(query.present, 'string');
    assert.deepEqual(query.type, {kind: 'string'});
    assert.equal(query.arg.k, 'optionalIndex');
    const projected = projectLinkedProgramExpression(query, program.declarations);
    assert.equal(projected.k, 'typeOf'); assert.equal(projected.arg.k, 'index');
    assert.equal(projected.arg.optional, true);
    assert.equal(Object.hasOwn(projected, 'present'), false, 'derived tag is not trusted/copied');
    assert.deepEqual(observe(emitJavaScript(program).text), expected);
    assert.deepEqual(emitRust(program), originalRust);
    query.arg.index.right.value = '11';
    const before = structuredClone(program);
    const rechecked = recheckLinkedTypedProgram(program, {wholeProgram: true});
    assert.deepEqual(program, before, 'the coherent edit is not normalized by rechecking');
    const edited = encodeTypedProgramLinks(rechecked);
    for (const [, network] of forms(edited)) {
      const restored = decodeTypedProgramLinks(network, edited.root), before = structuredClone(restored);
      const rechecked = recheckLinkedTypedProgram(restored, {wholeProgram: true});
      assert.deepEqual(restored, before, 'the restored coherent edit is not normalized by rechecking');
      assert.deepEqual(observe(emitJavaScript(rechecked).text), {output: ['undefined', 'undefined'], failure: null});
    }
  }
});

test('whole-program rechecking refuses forged optional query tags types arguments and extra ownership meaning', () => {
  const baseline = savedPrograms(source)[0], packet = encodeTypedProgramLinks(baseline);
  for (const [format, network] of forms(packet)) for (const change of [
    query => { query.present = 'number'; },
    query => { query.present = null; },
    query => { delete query.present; },
    query => { query.type = {kind: 'bool'}; },
    query => { query.arg = {k: 'lit', type: {kind: 'string'}, value: 'why'}; },
    query => { query.arg.array.type = {kind: 'array', element: {kind: 'bool'}}; },
    query => { query.opaqueBinding = true; },
  ]) {
    const program = decodeTypedProgramLinks(network, packet.root), query = program.declarations.get('query').body;
    change(query); const before = structuredClone(program);
    assert.throws(() => recheckLinkedTypedProgram(program, {wholeProgram: true}), Error, format);
    assert.deepEqual(program, before, 'a failed whole check does not silently repair the editable graph');
  }
});

test('optional query source-free replay retains ordered once-only reads absence and throws', () => {
  for (const [array, index, abort, expected] of [
    ['["why"]', '0', false, {output: ['values', 'index', 'string'], failure: null}],
    ['[]', '0', false, {output: ['values', 'index', 'undefined'], failure: null}],
    ['["why"]', '-1', false, {output: ['values', 'index', 'undefined'], failure: null}],
    ['["why"]', '0.5', false, {output: ['values', 'index', 'undefined'], failure: null}],
    ['["why"]', 'NaN', false, {output: ['values', 'index', 'undefined'], failure: null}],
    ['["why"]', '0', true, {output: ['values', 'index'], failure: {name: 'Error', message: 'stopped'}}],
  ]) {
    const source = '/** @returns {string[]} */ function values(){console.log("values");return ' + array + ';}'
      + '/** @param {number} value @returns {number} */ function index(value){console.log("index");'
      + (abort ? 'if(value<0)throw new Error("stopped");' : '') + 'return value;}'
      + '/** @returns {string} */ function probe(){return typeof values()[index('
      + (abort ? '-1' : index) + ')];}console.log(probe());';
    assert.deepEqual(observe(source), expected);
    const programs = savedPrograms(source), rust = emitRust(programs[0]);
    for (const program of programs) {
      assert.deepEqual(observe(emitJavaScript(program).text), expected);
      assert.deepEqual(emitRust(program), rust, 'full Rust emission is retained; native execution remains CI-only');
    }
  }
});
