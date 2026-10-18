import assert from 'node:assert/strict';
import test from 'node:test';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {acceptCompleteDeclarationSourceInterval} from '../src/source-span-rules.js';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {encodeTypedProgramLinks,decodeTypedProgramLinks} from '../src/typed-program-links.js';
import {recheckLinkedTypedProgram} from '../src/typed-program-checked-projection.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';

const fixture=readProjectData(new URL('../../parity/fixtures/complete-declaration-source-ranges.lino',import.meta.url));

test('complete declaration intervals retain exact safe offset limits and ordered endpoints',()=>{
 let compared=0;
 for(const [start,prefix,end]of [[0,0,0],[0,0,1],[0,1,1],[1,1,2],[25,31,62],
  [Number.MAX_SAFE_INTEGER-2,Number.MAX_SAFE_INTEGER-1,Number.MAX_SAFE_INTEGER],
  [Number.MAX_SAFE_INTEGER,Number.MAX_SAFE_INTEGER,Number.MAX_SAFE_INTEGER]]){
  assert.equal(acceptCompleteDeclarationSourceInterval(start,prefix,end),true);
  compared+=1;
 }
 for(const invalid of [-Infinity,-1,0.5,Number.MAX_SAFE_INTEGER+1,Infinity,NaN])for(const slot of [0,1,2]){
  const offsets=[0,1,2];offsets[slot]=invalid;
  assert.equal(acceptCompleteDeclarationSourceInterval(...offsets),false);compared+=1;
 }
 for(const offsets of [[2,1,3],[0,2,1],[1,1,0]]){
  assert.equal(acceptCompleteDeclarationSourceInterval(...offsets),false);compared+=1;
 }
 assert.equal(compared,28);
});

test('actual callable forms retain full UTF16 declaration ranges and unchanged independent observations',async()=>{
 assert.equal(fixture.schemaVersion,1);assert.equal(fixture.coordinateUnit,'UTF-16 code units');assert.equal(fixture.cases.length,14);
 let supported=0,refused=0;
 for(const row of fixture.cases){
  const original=await import('data:text/javascript;base64,'+Buffer.from(row.source).toString('base64'));
  assert.deepEqual(original.retained(...row.arguments),row.expectedValue,row.id+' original source');
  if(row.expectedError){
   assert.throws(()=>parseJavaScript(row.source,row.context??undefined),error=>{
    assert.deepEqual({kind:error.kind,reason:error.reason,span:error.span},row.expectedError);return true;
   });refused+=1;continue;
  }
  const surface=parseJavaScript(row.source,row.context??undefined),owners=surface.items.filter(item=>item.name==='retained');
  assert.equal(owners.length,1);assert.deepEqual(owners[0].span,row.expectedSpan,row.id);
  const checked=checkProgram(surface),javascript=emitJavaScript(checked),call=runInNewContext(javascript.text+'\n;retained;',{}, {timeout:1000});
  assert.deepEqual(call(...row.arguments),row.expectedValue,row.id+' complete checked emission');
  const mappings=emitRust(checked).mappings.filter(item=>item.kind==='function'&&item.source==='retained');
  assert.equal(mappings.length,1);assert.deepEqual(mappings[0].sourceSpan,row.expectedSpan);
  supported+=1;
 }
 assert.deepEqual([supported,refused],[13,1]);
});

test('complete callable provenance survives both source-free archives with every target emission field',()=>{
 let replays=0;
 for(const row of fixture.cases){
  if(row.expectedError)continue;
  const checked=checkProgram(parseJavaScript(row.source,row.context??undefined)),rust=emitRust(checked),javascript=emitJavaScript(checked);
  const packet=encodeTypedProgramLinks(checked),text=packet.network.toLino(),binary=encodeLinkNetworkBinary(packet.network);
  for(const program of[decodeTypedProgramLinks(LinkNetwork.fromLino(text),packet.root),decodeTypedProgramLinks(decodeLinkNetworkBinary(binary),packet.root)]){
   const restored=recheckLinkedTypedProgram(program,{wholeProgram:true}),emitted=emitRust(restored);
   for(const field of ['text','definitions','preludes','mappings','assumptions','encodings'])assert.deepEqual(emitted[field],rust[field],row.id+' '+field);
   assert.deepEqual(emitJavaScript(restored),javascript);replays+=1;
  }
 }
 assert.equal(replays,26);
});
