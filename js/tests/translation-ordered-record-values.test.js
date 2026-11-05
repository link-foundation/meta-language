// Deferred graph/codec/product controls. No draft-only execution is allowed.
import assert from 'node:assert/strict';
import test from 'node:test';
import {execFileSync} from 'node:child_process';
import {readFileSync} from 'node:fs';
import {readStructuredDataLino} from '../src/structured-data-links.js';
import {checkProgram} from '../src/translation/check.js';
import {recheckLinkedTypedProgram} from '../src/typed-program-checked-projection.js';
import {encodeTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {prepareOrderedRecordValues} from '../src/translation/ordered-record-preparation.js';
import {threadOutput} from '../src/translation/output.js';
test('dynamic ordered record values retain every input, error, source declaration and codec',()=>{
 const fixture=readStructuredDataLino(readFileSync(new URL('../../parity/fixtures/ordered-record-values.lino',import.meta.url),'utf8'));
 const observed=readStructuredDataLino(execFileSync(process.execPath,['../experiments/ordered-record-values/produce.mjs'],{cwd:new URL('../',import.meta.url),maxBuffer:64*1024*1024}).toString('utf8'));
 assert.equal(observed.cases.length,fixture.cases.length);
 for(const [index,row]of fixture.cases.entries()){
  const expected=row.error?{kind:'error',...row.error}:{kind:'value',keys:row.keys,values:row.values};
  assert.equal(observed.cases[index].name,row.name);assert.deepEqual(observed.cases[index].observed,expected,row.name);
  if(!row.error)assert.deepEqual(observed.cases[index].original,expected,row.name);
 }
 const program=checkProgram(fixture.surface),original=structuredClone(program);
 for(let repeat=0;repeat<3;repeat+=1){assert.equal(recheckLinkedTypedProgram(program,{wholeProgram:true}),program);assert.deepEqual(program,original);}
 const packet=encodeTypedProgramLinks(program);assert.equal(packet.network.toLino(),observed.links);
 for(const network of [packet.network,LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]){
  assert.deepEqual(decodeTypedProgramLinks(network,packet.root),program);
  for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),observed.emissions[target]);
 }
 const prepared=prepareOrderedRecordValues(program);
 assert.ok([...prepared.declarations.keys()].some(name=>name.endsWith('.createJavaScriptRecordPropertyOrder')));
 assert.equal(threadOutput(prepared,'Lean').abortsThreaded,true);
 assert.deepEqual(program,original);
});
