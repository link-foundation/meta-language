import assert from 'node:assert/strict';
import { test } from 'node:test';
import { readFileSync } from 'node:fs';
import { createHash } from 'node:crypto';
import { runInNewContext } from 'node:vm';
import { DecoratorSet } from '../src/decorators.js';
import { LinkNetwork } from '../src/network.js';
import { ByteRange, LinkMetadata, LinkType } from '../src/primitives.js';
import { encodeLinkNetworkBinary, decodeLinkNetworkBinary, BinaryLinksError } from '../src/link-network-binary.js';
import { linkNetworkBinaryIntegerByte } from '../src/link-network-binary-rules.js';
import { transcodeLinkNetworkBinary } from '../src/link-network-binary-record-rules.js';
import { transcodeLinkNetworkBinary as previous } from '../../experiments/binary-metadata-buffer/prior-rules.mjs';
import { readProjectData } from '../scripts/project-data.mjs';

const fixture = readProjectData(new URL('../../parity/fixtures/link-network-binary.lino', import.meta.url));

test('complete graph reconstruction preserves IDs, cycles, metadata, exact caches and deleted allocation history', () => {
  const parse = LinkNetwork.parse;
  LinkNetwork.parse = () => { throw new Error('source parsing cannot supply a binary snapshot'); };
  let restored;
  try { restored = decodeLinkNetworkBinary(Uint8Array.from(fixture.version2Bytes)); } finally { LinkNetwork.parse = parse; }
  assert.equal(restored.toLino(), fixture.graph);
  assert.deepEqual([...restored._terms].map(([term, id]) => [term, id.asU64()]).sort(), [...fixture.termMap].sort());
  assert.deepEqual([...restored._conceptSyntax].sort(), [...fixture.selectedSyntax].sort());
  assert.equal(restored.findTerm('original').asU64(), 1);
  assert.equal(restored.findTerm('renamed'), undefined);
  assert.equal(restored.reconstructConcept('meaning', 'JavaScript'), 'selected syntax');
  assert.deepEqual([...encodeLinkNetworkBinary(restored)], fixture.version2Bytes);
  assert.equal(restored.insertLink().asU64(), fixture.nextIdentity);
});

test('legacy MLDSNP01 retains its recorded graph and registration semantics', () => {
  const restored = decodeLinkNetworkBinary(fixture.legacyVersion1Bytes);
  assert.deepEqual(restored.links(), LinkNetwork.fromLino(fixture.graph).links());
  assert.equal(restored.findTerm('original'), undefined);
  assert.equal(restored.findTerm('renamed'), undefined);
  assert.equal(restored.reconstructConcept('meaning', 'JavaScript'), undefined);
  assert.equal(restored._nextId, restored.links().at(-1).id().asU64() + 1);
  assert.equal(new TextDecoder().decode(encodeLinkNetworkBinary(restored).slice(0, 8)), 'MLDSNP02');
});

test('reloaded graph edits change byte output and survive another reconstruction', () => {
  const restored = decodeLinkNetworkBinary(fixture.version2Bytes);
  restored.link(1).setReferences([1, 2, 1]);
  restored.setTerm(2, 'edited linked relation');
  const changed = encodeLinkNetworkBinary(restored);
  assert.notDeepEqual([...changed], fixture.version2Bytes);
  assert.deepEqual(decodeLinkNetworkBinary(changed).link(1).references().map(Number), [1, 2, 1]);
  assert.equal(decodeLinkNetworkBinary(changed).link(2).metadata().term, 'edited linked relation');
});

test('unsafe numeric IDs, spans, isolated surrogates and ambiguous cache keys are refused precisely', () => {
  const network = new LinkNetwork();
  const id = network.insertTypedPoint(LinkType.Concept, 'safe');
  network._nextId = Number.MAX_SAFE_INTEGER + 1;
  assert.throws(() => encodeLinkNetworkBinary(network), /unsafe or invalid next identity/);
  network._nextId = 2;
  network.link(id).setMetadata(network.link(id).metadata().withSpan({ byteRange: new ByteRange(0, Number.MAX_SAFE_INTEGER + 1), start: { row: 0, column: 0 }, end: { row: 0, column: 0 } }));
  assert.throws(() => encodeLinkNetworkBinary(network), /unsafe span value/);
  network.link(id).setMetadata(LinkMetadata.new().withTerm('\uD800'));
  assert.throws(() => encodeLinkNetworkBinary(network), /isolated surrogate/);
  network.link(id).setMetadata(LinkMetadata.new());
  network._conceptSyntax.set('a\0b\0c', 'selected');
  assert.throws(() => encodeLinkNetworkBinary(network), /ambiguous concept-syntax/);
});

test('invalid framing, truncation, trailing bytes and hostile counts are bounded', () => {
  const bytes = Uint8Array.from(fixture.version2Bytes);
  const header = bytes.slice(); header[0] = 0;
  assert.throws(() => decodeLinkNetworkBinary(header), /invalid archive header/);
  const version = bytes.slice(); version[7] = 51;
  assert.throws(() => decodeLinkNetworkBinary(version), /unsupported archive version/);
  assert.throws(() => decodeLinkNetworkBinary(bytes.slice(0, -1)), BinaryLinksError);
  assert.throws(() => decodeLinkNetworkBinary([...bytes, 0]), /trailing bytes/);
  const count = bytes.slice(); count.fill(255, 16, 24);
  assert.throws(() => decodeLinkNetworkBinary(count), /unsafe record count/);
  assert.throws(() => decodeLinkNetworkBinary(bytes, { byteLimit: 32 }), /byte resource limit/);
  assert.throws(() => encodeLinkNetworkBinary(new LinkNetwork(), { byteLimit: 16 }), /byte resource limit/);
  assert.throws(() => encodeLinkNetworkBinary(decodeLinkNetworkBinary(bytes), { recordLimit: 1 }), /record resource limit/);
  assert.throws(() => decodeLinkNetworkBinary([77, 76, 68, 83, 78, 80, 48, 50, 256]), BinaryLinksError);
});

test('both shared schemas agree and the complete portable framing source is admitted by the checked translator', async () => {
  assert.equal(readFileSync(new URL('../src/data/link-network-binary-schema.lino', import.meta.url), 'utf8'),
    readFileSync(new URL('../../rust/src/data/link-network-binary-schema.lino', import.meta.url), 'utf8'));
  const { parseJavaScript } = await import('../src/translation/javascript.js');
  const { checkProgram } = await import('../src/translation/check.js');
  const source = readFileSync(new URL('../src/link-network-binary-rules.js', import.meta.url), 'utf8');
  const program = checkProgram(parseJavaScript(source, { referenceSemantics: false }));
  assert.ok(program.items.length >= 8);
  assert.equal(transcodeLinkNetworkBinary([], true, 1, 32).startsWith('-3,'), true);
});

function createBinaryIdentityControl(version,identities){
 const record=identity=>[identity,0,1,0,0,0,0,0,0,0,0,0];
 return [version,...(version===2?[1000]:[]),identities.length,...identities.flatMap(record),...(version===2?[0,0]:[])];
}
function readBinaryControlBytes(encoded){return Array.from({length:encoded.length/2},(_,index)=>Number.parseInt(encoded.slice(index*2,index*2+2),16));}
function observeBinaryStringWork(operation){
 const includes=String.prototype.includes,slice=String.prototype.slice;let searchCount=0,sliceCharacters=0;
 try{
  String.prototype.includes=function(needle,...rest){if(typeof needle==='string'&&/^,\d+,$/u.test(needle))searchCount++;return includes.call(this,needle,...rest);};
  String.prototype.slice=function(...args){sliceCharacters+=this.length;return slice.call(this,...args);};
  const result=operation();return {result,searchCount,sliceCharacters};
 }finally{String.prototype.includes=includes;String.prototype.slice=slice;}
}

test('identity framing preserves unsorted records and exact duplicate refusals',()=>{
 const baseline=readFileSync(new URL('../../experiments/binary-metadata-buffer/prior-rules.mjs',import.meta.url));
 assert.equal(createHash('sha256').update(baseline).digest('hex'),'4f0debafedcc6e1985ccad8ed0dd673261e511e705d4d7535db008d446cf11a4');
 for(const version of[1,2])for(const identities of[[],[1,2,3],[3,1,2],[9,2,5],[1,2,1],[9,2,9],[5,1,5],[3,2,1,2]]){
  const input=createBinaryIdentityControl(version,identities),before=[...input],expected=previous(input,false,1000,65536);
  assert.equal(transcodeLinkNetworkBinary(input,false,1000,65536),expected);assert.deepEqual(input,before);
  if(!expected.startsWith('-')){const bytes=readBinaryControlBytes(expected);assert.equal(transcodeLinkNetworkBinary(bytes,true,1000,65536),previous(bytes,true,1000,65536));}
 }
 for(const version of[1,2])for(const record of[1,2]){
  const input=createBinaryIdentityControl(version,[1,2,3]),bytes=readBinaryControlBytes(previous(input,false,1000,65536)),offset=(version===2?24:16)+record*34;
  for(let index=0;index<8;index++)bytes[offset+index]=linkNetworkBinaryIntegerByte(1,index);
  const expected=previous(bytes,true,1000,65536);assert.ok(expected.startsWith('-8,'));assert.equal(transcodeLinkNetworkBinary(bytes,true,1000,65536),expected);
 }
});

test('sorted identity framing avoids prefix searches and confines metadata patching to one record',()=>{
 const count=64,input=createBinaryIdentityControl(2,Array.from({length:count},(_,index)=>index+1));
 const old=observeBinaryStringWork(()=>previous(input,false,1000,65536)),actual=observeBinaryStringWork(()=>transcodeLinkNetworkBinary(input,false,1000,65536));
 assert.equal(actual.result,old.result);assert.equal(old.searchCount,count);assert.equal(actual.searchCount,0);
 assert.ok(actual.sliceCharacters<=count*100,'only bounded local metadata participates in patching');
 assert.ok(old.sliceCharacters>actual.sliceCharacters*8,'the frozen implementation slices its accumulating whole prefix');
});

test('record local metadata keeps byte limits and failure precedence identical',()=>{
 for(const archive of[fixture.legacyVersion1Bytes,fixture.version2Bytes]){
  const flat=previous(archive,true,1000,65536).slice(0,-1).split(',').map(Number);
  for(const limit of[16,archive.length-1,archive.length,archive.length+1])for(const binary of[false,true]){
   const input=binary?archive:flat;assert.equal(transcodeLinkNetworkBinary(input,binary,1000,limit),previous(input,binary,1000,limit));
  }
  for(let index=0;index<flat.length;index++)for(const value of[NaN,-1,0.5,256]){
   const input=[...flat];input[index]=value;assert.equal(transcodeLinkNetworkBinary(input,false,1000,65536),previous(input,false,1000,65536));
  }
  for(let end=0;end<archive.length;end++){const input=archive.slice(0,end);assert.equal(transcodeLinkNetworkBinary(input,true,1000,65536),previous(input,true,1000,65536));}
 }
});

test('binary record generator paths relocate with the actual repository', () => {
  const profile = DecoratorSet.fromLino(readFileSync(new URL('../../parity/self-translation/link-network-binary-record-generator-adapter.lino', import.meta.url), 'utf8'));
  const action = profile.decorators.flatMap(row => row.actions).find(row => row.to?.startsWith('const styles = new URL('));
  assert.ok(action);
  assert.ok(action.to.includes('import.meta.url'));
  assert.ok(!action.to.includes('file://'));
  for (const base of ['file:///one/project/js/scripts/generate-link-network-binary-rules.mjs', 'file:///another%20checkout/%CE%B1/js/scripts/generate-link-network-binary-rules.mjs']) {
    const actual = runInNewContext(action.to.replaceAll('import.meta.url', 'baseUrl') + '\nstyles.href;', { URL, baseUrl: base }, { timeout: 1000 });
    assert.equal(actual, new URL('../../parity/self-translation/link-network-binary-record-rules-decorators.lino', base).href);
  }
});
