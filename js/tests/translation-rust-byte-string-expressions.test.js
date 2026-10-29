// Deferred controls. Source authors never import or execute this module.
import assert from 'node:assert/strict';
import test from 'node:test';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {createRustSourceOwnerProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import * as rules from '../src/translation/rust-byte-string-expression-rules.js';
const fixture=readProjectData(new URL('../../parity/fixtures/rust-byte-string-expressions.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
for(const [index,row]of fixture.payloads.entries())test('exact byte payload '+index,()=>assert.deepEqual(rules.readRustByteStringExpressionBytes(row.payload,row.suffix),row.expected));
for(const [index,row]of fixture.coercions.entries())test('static array coercion '+index,()=>assert.equal(rules.acceptRustStaticByteArrayCoercion(row.actual,row.expected,row.borrowed),row.accepted));
for(const [index,row]of fixture.owners.entries())test('fixed array source identity '+index,()=>assert.equal(rules.acceptRustStaticByteArrayOwnerEquality(row.model,row.leftKind,row.leftPath,row.rightKind,row.rightPath),row.expected));
for(const [index,row]of fixture.views.entries())test('explicit static byte slice '+index,()=>assert.equal(rules.acceptRustStaticByteArraySliceView(row.method,row.count,row.model,row.proven,row.layers,row.references),row.expected));
for(const row of fixture.reads)test('static array read model '+row.model,()=>assert.equal(rules.readRustStaticByteArrayReadModel(row.model),row.expected));
function arrays(node,result=[]){if(!node||typeof node!=='object')return result;if(node.k==='array')result.push(node);for(const[key,value]of Object.entries(node)){if(key==='type')continue;if(Array.isArray(value))for(const item of value)arrays(item,result);else if(value&&typeof value==='object')arrays(value,result);}return result;}
for(const row of fixture.cases)test(row.name,()=>{
 const original=row.source+'fn main(){}',concrete=LinkNetwork.parse(original,'Rust');for(const network of [concrete,LinkNetwork.fromLino(concrete.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(concrete))])assert.equal(network.reconstructText(),original);
 const packet=createRustSourceOwnerProgramLinks(original),program=decodeTypedProgramLinks(packet.network,packet.root);
 for(const network of [packet.network,LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]){
  const restored=decodeTypedProgramLinks(network,packet.root);assert.deepEqual([...restored.declarations].map(([name,entry])=>[name,entry.sourceSignature]),[...program.declarations].map(([name,entry])=>[name,entry.sourceSignature]));
  if(row.bytes){const actual=arrays(restored.declarations.get('result').body).find(node=>node.items.length===row.bytes.length);assert.ok(actual,'complete literal byte array');assert.deepEqual(actual.type.element,{kind:'fixed',bits:8,signed:false});assert.deepEqual(actual.items.map(value=>Number(value.value)),row.bytes);}
  for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));
  const emission=emitTypedProgramLinks(network,packet.root,'JavaScript'),name=emission.mappings.find(mapping=>mapping.kind==='function'&&mapping.source==='result').target;assert.equal(String(runInNewContext(emission.code.replace(/^export /gmu,'')+'\n'+name+'()',{}, {timeout:1000})),row.value);
 }
});
for(const row of fixture.refusals)test(row.name,()=>assert.throws(()=>createRustSourceOwnerProgramLinks(row.source),error=>error.kind===row.kind&&error.reason===row.reason));
