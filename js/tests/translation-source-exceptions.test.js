// Future independent source controls. No test execution during the whole draft.
import assert from 'node:assert/strict';
import test from 'node:test';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {createSourceExceptionProgramLinks} from '../src/source-exception-program.js';
import {createBigIntegerConversionProgramLinks} from '../src/big-integer-conversion-program.js';
import {createJavaScriptStringUnitProgramLinks} from '../src/javascript-string-unit-program.js';
import {encodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node);
const observe=(call,row)=>{try{return{kind:'value',value:String(call())};}catch(error){return{kind:'abort',value:row.errorClass===undefined?error.message:error.message.toWellFormed(),...(row.errorClass===undefined?{}:{errorClass:error.name,messageUnits:Array.from({length:error.message.length},(_,at)=>error.message.charCodeAt(at)),messagePresent:Object.hasOwn(error,'message')})};}};
for(const [fixtureName,createDefinition] of [['source-exceptions',createSourceExceptionProgramLinks],['big-integer-source',createBigIntegerConversionProgramLinks],['javascript-string-units',createJavaScriptStringUnitProgramLinks]])test(fixtureName+' retains every source value exception field and complete codec/emission result',()=>{
 const fixture=readProjectData(new URL('../../parity/fixtures/'+fixtureName+'.lino',import.meta.url));
 const program=checkProgram(parseJavaScript(fixture.source)),packet=encodeTypedProgramLinks(program),output=emitJavaScript(program);
 for(const actual of [packet,createDefinition()])for(const target of ['JavaScript','Rust','Lean','Rocq']){
  const expected=emitTypedProgramLinks(actual.network,actual.root,target);
  for(const network of [LinkNetwork.fromLino(actual.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(actual.network))])assert.deepEqual(emitTypedProgramLinks(network,actual.root,target),expected);
 }
 const names=fixture.cases.map(row=>output.mappings.find(mapping=>mapping.kind==='function'&&mapping.source===row.entry).target);
 const functions=runInNewContext('"use strict";\n'+output.text.replace(/^export /gmu,'')+'\n['+names.join(',')+'];');
 for(const [index,row] of fixture.cases.entries()){
  const expected={kind:row.abort===null?'value':'abort',value:row.abort??row.value,...(row.errorClass===undefined?{}:{errorClass:row.errorClass,messageUnits:row.messageUnits,messagePresent:row.messagePresent})};
  assert.deepEqual(observe(()=>runInNewContext('"use strict";\n'+row.source.replace(/^export /gmu,'')+'\ncompute();',{}, {timeout:1000}),row),expected,row.name);
  assert.deepEqual(observe(()=>functions[index](),row),expected,row.name);
 }
});
