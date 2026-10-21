import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {runInNewContext} from 'node:vm';
import {checkProgram} from '../src/translation/check.js';
import {parseJavaScript} from '../src/translation/javascript.js';
import {encodeTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {projectLinkedProgramExpression} from '../src/typed-program-expression-projection.js';
import {recheckLinkedTypedProgram} from '../src/typed-program-checked-projection.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {TRUTHY_PROJECTION_CASES,TRUTHY_PROJECTION_CHECKED_CASES,TRUTHY_PROJECTION_LITERAL_INPUT,
 TRUTHY_PROJECTION_HOST_PARAMETER_SOURCE,TRUTHY_PROJECTION_HOST_PARAMETER_VALUES} from '../scripts/truthy-projection-fixtures.mjs';
import {generateTruthyProjectionSchema} from '../scripts/generate-truthy-projection-schema.mjs';
import {deriveTruthyProjectionSchema,verifyTruthyProjectionSchemaPacket,
 restoreTruthyProjectionHistoricalSource,readTruthyProjectionSchemaPacket} from '../scripts/truthy-projection-schema.mjs';
import {restoreCollectionSizeProjectionSchema,readCollectionSizeProjectionSchemaPacket,
 verifyCollectionSizeProjectionSchemaPacket} from '../scripts/collection-size-projection-schema.mjs';

const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node);
function observe(source){const lines=[];runInNewContext(source,{console:{log:(...values)=>lines.push(values.map(String).join(' '))}},{timeout:1000});return lines;}
function* archives(network){yield LinkNetwork.fromLino(network.toLino());yield decodeLinkNetworkBinary(encodeLinkNetworkBinary(network));}
function truthy(program){
 const found=[],seen=new WeakSet();
 function walk(value){if(!value||typeof value!=='object'||seen.has(value))return;seen.add(value);if(value.k==='truthy')found.push(value);for(const child of value instanceof Map?value.values():Object.values(value))walk(child);}
 walk(program);assert.ok(found.length,'actual source checker creates a Boolean conversion');return found;
}

test('all original literal oracles remain and unsupported lone escapes retain the exact existing boundary',()=>{
 for(const input of TRUTHY_PROJECTION_CASES)assert.deepEqual(observe(input.source),input.expected,input.id+' complete original JavaScript oracle');
 const source=TRUTHY_PROJECTION_LITERAL_INPUT.source;
 assert.throws(()=>parseJavaScript(source),error=>error.kind==='unsupported'&&error.reason.includes('not a scalar value')
  &&error.span.start===source.indexOf('\\ud800'));
 assert.deepEqual(TRUTHY_PROJECTION_HOST_PARAMETER_VALUES.slice(0,6).map(input=>String(input.expected)),TRUTHY_PROJECTION_LITERAL_INPUT.expected);
});

test('complete original string parameter preserves the same values and lone surrogates through both checked semantic codecs',()=>{
 const source=TRUTHY_PROJECTION_HOST_PARAMETER_SOURCE;
 const original=runInNewContext(source+'\nprobe;',{}, {timeout:1000});
 const values=TRUTHY_PROJECTION_HOST_PARAMETER_VALUES.map(input=>String.fromCharCode(...input.units));
 for(const [index,value]of values.entries())assert.equal(original(value),TRUTHY_PROJECTION_HOST_PARAMETER_VALUES[index].expected);
 // The first six values are exactly those evaluated by the retained complete
 // literal input, including its otherwise unsupported lone high surrogate.
 const literalValues=runInNewContext('["","0","false","\\u0000","\\ud800","😀"]',{}, {timeout:1000});
 assert.deepEqual(values.slice(0,6),Array.from(literalValues));
 const program=checkProgram(parseJavaScript(source));program.utf16Strings=true;
 assert.equal(recheckLinkedTypedProgram(program,{wholeProgram:true}),program);
 assert.equal(truthy(program).length,1);
 const packet=encodeTypedProgramLinks(program),rust=emitTypedProgramLinks(packet.network,packet.root,'Rust').code;
 for(const network of archives(packet.network)){
  const restored=decodeTypedProgramLinks(network,packet.root);assert.deepEqual(restored,program);
  assert.equal(restored.utf16Strings,true);assert.equal(recheckLinkedTypedProgram(restored,{wholeProgram:true}),restored);
  const emitted=emitTypedProgramLinks(network,packet.root,'JavaScript').code;
  const checked=runInNewContext(emitted+'\nprobe;',{}, {timeout:1000});
  for(const [index,value]of values.entries())assert.equal(checked(value),TRUTHY_PROJECTION_HOST_PARAMETER_VALUES[index].expected);
  assert.equal(emitTypedProgramLinks(network,packet.root,'Rust').code,rust);
 }
});

test('existing source Boolean conversions recheck exactly and preserve every independent result through both semantic codecs',()=>{
 for(const input of TRUTHY_PROJECTION_CHECKED_CASES){
  assert.deepEqual(observe(input.source),input.expected,input.id+' original JavaScript oracle');
  const program=checkProgram(parseJavaScript(input.source));program.utf16Strings=true;
  for(const operation of truthy(program)){
   const projected=projectLinkedProgramExpression(operation);
   assert.equal(projected.k,'booleanValue');assert.ok(projected.arg);assert.equal(Object.hasOwn(projected,'then'),false);
  }
  const before=structuredClone(program);assert.equal(recheckLinkedTypedProgram(program,{wholeProgram:true}),program);assert.deepEqual(program,before);
  const packet=encodeTypedProgramLinks(program),rust=emitTypedProgramLinks(packet.network,packet.root,'Rust').code;
  for(const network of archives(packet.network)){
   const restored=decodeTypedProgramLinks(network,packet.root);assert.deepEqual(restored,program,input.id+' full IR unchanged');
   assert.equal(recheckLinkedTypedProgram(restored,{wholeProgram:true}),restored);
   assert.deepEqual(observe(emitTypedProgramLinks(network,packet.root,'JavaScript').code),input.expected,input.id);
   assert.equal(emitTypedProgramLinks(network,packet.root,'Rust').code,rust);
  }
 }
});

test('Boolean conversion projection cannot hide forged owners types operands operations or result wrappers',()=>{
 const source=TRUTHY_PROJECTION_CASES[0].source,original=checkProgram(parseJavaScript(source));
 for(const change of [
  (program,value)=>{value.type={kind:'string'};},
  (program,value)=>{value.arg.name='unbound';},
  (program,value)=>{value.arg.type={kind:'string'};},
  (program,value)=>{value.op='arbitrary';},
  (program,value)=>{value.unrepresentedPolicy=true;},
  (program,value)=>{value.arg={k:'lit',value:true,type:{kind:'bool'}};},
 ]){
  const changed=structuredClone(original);change(changed,truthy(changed)[0]);
  assert.throws(()=>recheckLinkedTypedProgram(changed,{wholeProgram:true}));
  assert.throws(()=>{recheckLinkedTypedProgram(changed,{wholeProgram:true});return encodeTypedProgramLinks(changed);});
 }
 const packet=encodeTypedProgramLinks(original);
 for(const reload of archives(packet.network)){
  const owner=reload.fieldRelations().find(row=>row.label==='construct kind'&&reload.link(row.child).metadata().definition==='truthy');
  assert.ok(owner);reload.insertFieldRelation(owner.parent,'unrepresented policy',owner.child);
  for(const network of archives(reload))assert.throws(()=>decodeTypedProgramLinks(network,packet.root),/unrecognized semantic field/u);
 }
});

test('normal Boolean projection successor preserves every complete predecessor and historical collection view',()=>{
 const packet=generateTruthyProjectionSchema({check:true}),proof=verifyTruthyProjectionSchemaPacket(packet);
 const collection=readCollectionSizeProjectionSchemaPacket(),old=verifyCollectionSizeProjectionSchemaPacket(collection);
 assert.equal(deriveTruthyProjectionSchema(proof.sources[0].previous.source),proof.sources[0].current.source);
 for(const row of proof.sources){
  const bytes=readFileSync(new URL('../../'+row.file,import.meta.url));assert.equal(bytes.toString('utf8'),row.current.source);
  assert.equal(restoreTruthyProjectionHistoricalSource(bytes,row.file),row.previous.source);
  assert.equal(restoreTruthyProjectionHistoricalSource(Buffer.from(row.previous.source),row.file),row.previous.source);
  assert.throws(()=>restoreTruthyProjectionHistoricalSource(Buffer.concat([bytes,Buffer.from('\n')]),row.file));
 }
 for(const row of proof.sources.slice(0,2)){
  assert.equal(row.previous.source,old.current);
  assert.equal(restoreCollectionSizeProjectionSchema(Buffer.from(row.current.source),row.file),old.predecessor);
 }
 assert.throws(()=>restoreTruthyProjectionHistoricalSource(Buffer.from(proof.sources[0].current.source),'unregistered/schema.lino'));
 for(const change of [
  value=>{value.extensions.expressionProjections.truthy.construct='if';},
  value=>{value.extensions.expressionFields.truthy.push('op');},
  value=>{value.sources[0].current.source+='\n';},
  value=>{value.sources[0].previous.sha256='0'.repeat(64);},
  value=>{value.sources[2].certificate.blob.sha256='0'.repeat(64);},
  value=>{value.sources[2].current.source+='\n';},
  value=>{value.unregisteredAuthority=true;},
 ]){const changed=structuredClone(packet);change(changed);assert.throws(()=>verifyTruthyProjectionSchemaPacket(changed));}
 assert.equal(createHash('sha256').update(proof.sources[0].previous.source).digest('hex'),'d34bf62c4fd6114e5cdaa8a78558de01b83d373175185ee6a94b56114b80599f');
 assert.deepEqual(readTruthyProjectionSchemaPacket(),packet,'readers return independently checked complete data');
});
