import {createJavaScriptOwnPropertyProgramLinks} from '../src/javascript-own-property-program.js';
import {createJavaScriptLiteralPropertyProgramLinks} from '../src/javascript-literal-property-program.js';
import {createJavaScriptNullishFieldProgramLinks} from '../src/javascript-nullish-field-program.js';
import {createJavaScriptOptionalChainProgramLinks} from '../src/javascript-optional-chain-program.js';
import {createJavaScriptOptionalCallProgramLinks} from '../src/javascript-optional-call-program.js';
import {createJavaScriptPropertyCallProgramLinks} from '../src/javascript-property-call-program.js';
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
import {createStringConstructionProgramLinks} from '../src/string-construction-program.js';
import {createJavaScriptComparisonProgramLinks} from '../src/javascript-comparison-program.js';
import {createArrayBindingProgramLinks} from '../src/array-binding-program.js';
import {createOptionalValueProgramLinks} from '../src/optional-value-program.js';
import {createJavaScriptRecordParameterProgramLinks} from '../src/javascript-record-parameter-program.js';
import {createUtf16TextProgramLinks} from '../src/utf16-text-program.js';
import {createScalarAssignmentProgramLinks} from '../src/scalar-assignment-program.js';
import {encodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {threadOutput} from '../src/translation/output.js';
import {hasSourceExceptionPayload} from '../src/translation/source-exception-observation.js';
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node);
const observe=(call,row)=>{try{return{kind:'value',value:String(call())};}catch(error){return{kind:'abort',value:row.errorClass===undefined?error.message:error.message.toWellFormed(),...(row.errorClass===undefined?{}:{errorClass:error.name,messageUnits:Array.from({length:error.message.length},(_,at)=>error.message.charCodeAt(at)),messagePresent:Object.hasOwn(error,'message')})};}};
for(const [fixtureName,createDefinition] of [['source-exceptions',createSourceExceptionProgramLinks],['numeric-operation-errors',createSourceExceptionProgramLinks],['big-integer-source',createBigIntegerConversionProgramLinks],['big-integer-division',createBigIntegerConversionProgramLinks],['exponentiation',createBigIntegerConversionProgramLinks],['primitive-comparisons',createJavaScriptComparisonProgramLinks],['binding-defaults',createArrayBindingProgramLinks],['record-parameter-writes',createJavaScriptRecordParameterProgramLinks],['sequence-expressions',createScalarAssignmentProgramLinks],['string-split',createUtf16TextProgramLinks],['string-replacement',createUtf16TextProgramLinks],['string-unit-calls',createJavaScriptPropertyCallProgramLinks],['javascript-optional-calls',createJavaScriptOptionalCallProgramLinks],['javascript-optional-property-calls',createJavaScriptOptionalCallProgramLinks],['javascript-optional-chains',createJavaScriptOptionalChainProgramLinks],['javascript-nullish-fields',createJavaScriptNullishFieldProgramLinks],['javascript-nullish-methods',createJavaScriptNullishFieldProgramLinks],['javascript-computed-reads',createJavaScriptNullishFieldProgramLinks],['javascript-indexed-properties',createJavaScriptLiteralPropertyProgramLinks],['javascript-own-properties',createJavaScriptOwnPropertyProgramLinks],['javascript-string-units',createJavaScriptStringUnitProgramLinks],['string-construction',createStringConstructionProgramLinks]])test(fixtureName+' retains every source value exception field and complete codec/emission result',()=>{
 const fixture=readProjectData(new URL('../../parity/fixtures/'+fixtureName+'.lino',import.meta.url));
 if(fixture.oracleArchitectures)assert.ok(fixture.oracleArchitectures.includes(process.arch),'uncovered source architecture '+process.arch);
 const program=checkProgram(parseJavaScript(fixture.source)),packet=encodeTypedProgramLinks(program),output=emitJavaScript(program);
 const inspectedCaptures=new WeakSet();
 const inspectCapture=value=>{
  if(!value||typeof value!=='object')return;
  if(inspectedCaptures.has(value))return;inspectedCaptures.add(value);
  if(value.k==='record'){
   const names=value.entries.map(entry=>entry.name);
   assert.deepEqual(names,[...names].sort(),'every checked record keeps canonical UTF16 field order');
   assert.deepEqual(value.type.fields.map(field=>field.name),names,'every record projection uses its actual canonical layout');
  }
  for(const child of value instanceof Map?value.values():Object.values(value))inspectCapture(child);
 };
 inspectCapture(program);
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

test('formal exception transport retains every component through nested calls without capturing source names',()=>{
 const fixture=readProjectData(new URL('../../parity/fixtures/source-exceptions.lino',import.meta.url));
 const original=checkProgram(parseJavaScript(fixture.source+'\nconsole.log(sourceExceptionNested0());'));
 for(const target of ['Lean','Rocq']){
  const threaded=threadOutput(original,target),pairs=[...threaded.declarations.values()].filter(entry=>entry.output);
  assert.ok(pairs.length>0);
  for(const pair of pairs){
   assert.ok(hasSourceExceptionPayload(pair));
   assert.deepEqual(pair.ctors[1].fields.map(field=>[field.name,field.type.kind]),[['output','output'],['message','string'],['className','string'],['messagePresent','bool']]);
   assert.ok(!original.declarations.has(pair.fullName),'generated pair captures '+pair.fullName);
  }
  const pairsByName=new Map(pairs.map(entry=>[entry.fullName,entry]));
  const visit=node=>{
   if(!node||typeof node!=='object')return;
   if(Array.isArray(node)){node.forEach(visit);return;}
   if(node.k==='ctor'&&pairsByName.get(node.data)?.ctors[1].name===node.ctor)assert.equal(node.args.length,4);
   if(node.k==='match')for(const arm of node.cases)if(pairsByName.get(arm.pattern.data)?.ctors[1].name===arm.pattern.ctor)assert.equal(arm.pattern.binds.length,4);
   for(const [key,value] of Object.entries(node))if(!['type','domain','span'].includes(key))visit(value);
  };
  for(const declaration of threaded.declarations.values())if(declaration.k==='fn')visit(declaration.body);
  const packet=encodeTypedProgramLinks(original),emitted=emitTypedProgramLinks(packet.network,packet.root,target);
  assert.match(emitted.code,/ml_source_exception_observation/);
 }
});

test('complete optional-value and Boolean definition graph retains every target and codec',()=>{
 const packet=createOptionalValueProgramLinks();
 for(const target of ['JavaScript','Rust','Lean','Rocq']){
  const expected=emitTypedProgramLinks(packet.network,packet.root,target);
  for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),expected);
 }
});
