import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { LinkNetwork } from '../src/network.js';
import { encodeLinkNetworkBinary, decodeLinkNetworkBinary } from '../src/link-network-binary.js';
import { encodeTypedProgramLinks, decodeTypedProgramLinks } from '../src/typed-program-links.js';
import { recheckLinkedTypedProgram } from '../src/typed-program-checked-projection.js';
import { readOwnedArrayAppendProofs } from '../src/translation/owned-array-append.js';
import { acceptOwnedArraySelfSliceRebinding } from '../src/translation/owned-array-append-rules.js';
import { generateOwnedArrayAppendRules, readOwnedArrayAppendSourceModule,
  readOwnedArraySelfSliceFixtures, readOwnedArraySelfSliceRefusals } from '../scripts/generate-owned-array-append-rules.mjs';

const check = source => checkProgram(parseJavaScript(source, { referenceSemantics: false }));
function versions() {
  const [major, minor] = process.versions.node.split('.').map(Number);
  assert.ok(major === 22 && minor >= 15 || major === 24,
    'uncovered Node version ' + process.versions.node + '; >=22.15 <23 || >=24 <25 required');
}
function observe(source) {
  const lines = [];
  vm.runInNewContext(source, { console: { log: (...args) => lines.push(args.map(String).join(' ')) } }, { timeout: 1000 });
  return lines.join('\n') + '\n';
}
const name = value => ({ k: 'name', path: [value] });
const number = value => ({ k: 'num', value: String(value) });
const slice = (receiver = name('values'), args = [number(0)]) => ({ k: 'method', op: 'slice', object: receiver, args });
const allocation = () => ({ s: 'let', name: 'values', value: { k: 'array', items: [{ value: number(1) }] } });
const append = () => ({ s: 'expr', expr: { k: 'method', op: 'push', object: name('values'), args: [number(2)] } });

test('self-slice decisions admit only complete method facts for one actual lexical owner', () => {
  versions();
  for (const count of [0, 1, 2]) assert.equal(acceptOwnedArraySelfSliceRebinding('method', 'slice', 'name', 1, true, count), true);
  assert.equal(acceptOwnedArraySelfSliceRebinding('copySlice', '', 'name', 1, true, 0), true);
  for (const facts of [
    ['app','slice','name',1,true,1], ['method','concat','name',1,true,1],
    ['method','slice','field',1,true,1], ['method','slice','name',2,true,1],
    ['method','slice','name',1,false,1], ['method','slice','name',1,true,-1],
    ['method','slice','name',1,true,0.5], ['method','slice','name',1,true,3],
    ['method','slice','name',1,true,NaN], ['method','slice','name',1,true,Infinity],
    ['copySlice','slice','name',1,true,0], ['copySlice','','name',1,true,1],
    ['copySlice','','name',1,false,0], ['copySlice','','name',2,true,0],
  ]) assert.equal(acceptOwnedArraySelfSliceRebinding(...facts), false, String(facts));
});

test('actual self-slice transport retains complete RHS reads and every prior escape', () => {
  versions();
  const rebind = value => ({ s: 'assign', name: 'values', value });
  const proof = entries => readOwnedArrayAppendProofs(entries).length;
  assert.equal(proof([allocation(), rebind(slice()), append()]), 1);
  assert.equal(proof([allocation(), rebind({k:'copySlice',object:name('values')}), append()]), 1, 'the actual zero-bound surface variant retains its owner');
  assert.equal(proof([allocation(), rebind(slice()), rebind(slice()), append()]), 1);
  assert.equal(proof([rebind(slice()), append()]), 0, 'no owner is invented');
  assert.equal(readOwnedArrayAppendProofs([rebind(slice()), append()], [{name:'values'}]).length, 0, 'parameter identity is not a fresh allocation');
  assert.equal(proof([allocation(), {s:'const',name:'alias',value:name('values')}, rebind(slice()), append()]), 0);
  assert.equal(proof([allocation(), rebind(slice(name('other'))), append()]), 0);
  assert.equal(proof([allocation(), rebind({k:'copySlice',object:name('other')}), append()]), 0);
  assert.equal(proof([allocation(), rebind({...slice(),op:'concat'}), append()]), 0);
  assert.equal(proof([allocation(), rebind(slice(name('values'), [name('values')])), append()]), 0, 'a slice argument read is never skipped');
  const captured = {k:'lambda',params:[],body:name('values')};
  assert.equal(proof([allocation(), rebind(slice(name('values'),[captured])), append()]), 0);
  const call = {k:'app',fn:{k:'name',path:['crate','unknown']},args:[name('values')]};
  assert.equal(proof([allocation(), rebind(slice(name('values'),[call])), append()]), 0);
  const alias = {s:'const',name:'alias',value:name('values')};
  assert.equal(proof([allocation(),{s:'while',cond:{k:'bool',value:true},body:[rebind(slice()),append(),alias]}]), 0, 'later aliases remain loop-carried escapes');
  assert.equal(proof([allocation(),{s:'if',cond:{k:'bool',value:true},then:[rebind(slice())],else:[rebind(name('other'))]},append()]), 0, 'an unsafe alternative is not forgotten at the join');
});

test('complete self-slice originals preserve branches loops lexical owners and bound effect order through both codecs', () => {
  versions();
  for (const fixture of readOwnedArraySelfSliceFixtures()) {
    assert.equal(observe(fixture.source), fixture.expected, fixture.id + ' independent original');
    const program = check(fixture.source);
    const native = emitRust(program, {arrayConsumption:true}).text;
    assert.match(native, /crate::ml_array::append\(/u, fixture.id);
    const packet = encodeTypedProgramLinks(program);
    for (const network of [LinkNetwork.fromLino(packet.network.toLino()),
      decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]) {
      const replay = recheckLinkedTypedProgram(decodeTypedProgramLinks(network, packet.root), {wholeProgram:true});
      assert.equal(observe(emitJavaScript(replay).text), fixture.expected, fixture.id);
      assert.equal(emitRust(replay, {arrayConsumption:true}).text, native, fixture.id + ' native codec output');
    }
  }
});

test('actual for-of readers preserve self-copy construction while aliasing readers remain unsafe', () => {
  versions();
  const reader = '/** @param {number[]} values @param {number} wanted @returns {boolean} */ function contains(values,wanted){for(const value of values){if(value===wanted)return true;}return false;}';
  const build = '/** @returns {number[]} */ function build(){let values=[1];while(values.length<4){values=values.slice();if(!contains(values,values.length+1))values.push(values.length+1);}return values;}\nconsole.log(build().join(","));';
  const source = reader + '\n' + build;
  assert.equal(observe(source), '1,2,3,4\n', 'independent original iteration and zero-bound copy');
  const program = check(source), packet = encodeTypedProgramLinks(program);
  const native = emitRust(program, {arrayConsumption:true}).text;
  for (const network of [LinkNetwork.fromLino(packet.network.toLino()), decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]) {
    const replay = recheckLinkedTypedProgram(decodeTypedProgramLinks(network, packet.root), {wholeProgram:true});
    assert.equal(observe(emitJavaScript(replay).text), '1,2,3,4\n');
    assert.equal(emitRust(replay, {arrayConsumption:true}).text, native);
  }
  const escaping = source.replace('for(const value of values)', 'const alias=values;for(const value of alias)');
  assert.equal(observe(escaping), '1,2,3,4\n', 'the identical independent result does not prove alias safety');
  assert.throws(() => check(escaping), /method call \.push\(\)/u);
  const live = 'function live(){let values=[1];for(const value of values){if(values.length<3)values.push(value+1);}return values.join(",");}\nconsole.log(live());';
  assert.equal(observe(live), '1,2,3\n', 'a live iterator observes actual array growth');
  assert.throws(() => check(live), /method call \.push\(\)/u, 'an active iterator alias is not a scalar reader call');
});

test('complete alias identity capture foreign assignment and escaping-bound originals retain their refusals', () => {
  versions();
  for (const fixture of readOwnedArraySelfSliceRefusals()) {
    assert.equal(observe(fixture.source), fixture.expected, fixture.id + ' independent original');
    assert.throws(() => check(fixture.source), undefined, fixture.id);
  }
});

test('normal ownership author retains the complete closure and the new shared decision', async () => {
  versions();
  const policy = await generateOwnedArrayAppendRules({ check: true, native: true });
  assert.ok(policy.decisions.includes('acceptOwnedArraySelfSliceRebinding'));
  assert.deepEqual(policy.selfSliceFixtures, readOwnedArraySelfSliceFixtures());
  assert.deepEqual(policy.selfSliceRefusals, readOwnedArraySelfSliceRefusals());
  const program = check(readOwnedArrayAppendSourceModule());
  const packet = encodeTypedProgramLinks(program);
  const output = emitRust(program, {arrayConsumption:true});
  assert.equal(output.definitions.length, program.declarations.size);
  for (const network of [LinkNetwork.fromLino(packet.network.toLino()), decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]) {
    const checked = recheckLinkedTypedProgram(decodeTypedProgramLinks(network, packet.root), {wholeProgram:true});
    const decision = vm.runInNewContext(emitJavaScript(checked).text + '\nacceptOwnedArraySelfSliceRebinding;');
    assert.equal(decision('method','slice','name',1,true,2), true);
    assert.equal(decision('method','slice','name',1,false,2), false);
    assert.equal(emitRust(checked,{arrayConsumption:true}).text, output.text);
  }
});
