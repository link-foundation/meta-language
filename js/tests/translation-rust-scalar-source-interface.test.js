// Deferred independent interface obligations and source-free controls.
import assert from 'node:assert/strict';
import test from 'node:test';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {createRustSourceOwnerProgramLinks} from '../src/rust-source-owner-program.js';
import {decodeTypedProgramLinks,emitTypedProgramLinks,encodeTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {emitRust} from '../src/translation/emit-rust.js';
import * as rules from '../src/translation/rust-scalar-source-interface-rules.js';
const fixture=readProjectData(new URL('../../parity/fixtures/rust-scalar-source-interface.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
const options={rustSourceHeaderEmissionModel:fixture.model};
const views=packet=>[packet.network,LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))];
for(const row of fixture.cases)test(row.name,()=>{
 const packet=createRustSourceOwnerProgramLinks(row.source),program=decodeTypedProgramLinks(packet.network,packet.root);
 const before=packet.network.toLino(),expected=emitTypedProgramLinks(packet.network,packet.root,'Rust',options);
 assert.ok(expected.code.includes('#[cfg(not(target_pointer_width = "64"))]'));assert.ok(expected.code.includes('compile_error!'));
 for(const network of views(packet)){
  assert.deepEqual([...decodeTypedProgramLinks(network,packet.root).declarations].map(([name,value])=>[name,value.sourceSignature]),[...program.declarations].map(([name,value])=>[name,value.sourceSignature]));
  assert.deepEqual(emitTypedProgramLinks(network,packet.root,'Rust',options),expected);
  for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));
 }
 assert.equal(packet.network.toLino(),before);
 const js=emitTypedProgramLinks(packet.network,packet.root,'JavaScript'),name=js.mappings.find(mapping=>mapping.kind==='function'&&mapping.source==='result').target;
 const value=runInNewContext(js.code.replace(/^export /gmu,'')+'\n'+name+'()',{}, {timeout:1000});
 if(row.resultType==='()')assert.equal(value,null);else assert.equal(String(value),row.expected);
 const portable=emitTypedProgramLinks(packet.network,packet.root,'Rust');assert.equal(portable.code,emitRust(program,{arrayConsumption:true}).text);
 assert.throws(()=>emitTypedProgramLinks(packet.network,packet.root,'JavaScript',options),/requires Rust target/u);
});
test('actual automatically translated scalar decisions cover every declared input',()=>{
 assert.equal(rules.acceptRustScalarSourceInterfaceModel(fixture.model),true);assert.equal(rules.acceptRustScalarSourceInterfaceModel('unknown'),false);
 for(const row of fixture.values)assert.equal(rules.readRustScalarValueType(row.kind,row.bits,row.signed),row.expected);
 for(const row of fixture.owners)assert.equal(rules.readRustScalarSourceType(row.value,row.kind,row.owner,row.paths,row.children),row.expected);
 for(const row of fixture.calls)assert.equal(rules.acceptRustScalarSourceCall(row.owned,row.arguments,row.result,row.static),row.expected);
 for(const row of fixture.budgets)assert.equal(rules.acceptRustScalarSourceBudget(row.visits),row.expected);
 for(const row of fixture.positions)assert.equal(rules.acceptRustScalarSourcePosition(row.model,row.source,row.invalid,row.borrowed),row.expected);
});
test('static copied borrows, borrowed results, old models and unrepresented qualifiers refuse',()=>{
 const row=fixture.cases.find(row=>row.name.startsWith('borrowed Boolean')),packet=createRustSourceOwnerProgramLinks(row.source);
 const changes=[
  [program=>{program.declarations.get('identity').sourceSignature.parameters[0].lifetimes=['static'];},/nonstatic copied borrows/u],
  [program=>{const result=program.declarations.get('identity').sourceSignature.result;result.wrappers=['reference'];result.lifetimes=[null];},/direct results/u],
  [program=>{program.declarations.get('identity').sourceSignature.model='Rust source function signature 1';},/version-two scalar owners/u],
 ];
 for(const [change,reason] of changes){const program=decodeTypedProgramLinks(packet.network,packet.root);change(program);const edited=encodeTypedProgramLinks(program);for(const network of views(edited))assert.throws(()=>emitTypedProgramLinks(network,edited.root,'Rust',options),reason);}
 const program=decodeTypedProgramLinks(packet.network,packet.root);program.declarations.get('identity').unsafe=true;assert.throws(()=>emitRust(program,{sourceHeaderModel:fixture.model}),/unrepresented source interface qualifiers/u);
 const nominal=createRustSourceOwnerProgramLinks('struct Value{value:usize}pub fn read(value:Value)->usize{value.value}fn main(){}');assert.throws(()=>emitTypedProgramLinks(nominal.network,nominal.root,'Rust',options),/original owned function declaration/u);
});
test('actual multi-argument calls retain one ordered binding per argument without capture',()=>{
 const row=fixture.cases.find(row=>row.name.startsWith('multiple borrowed')),packet=createRustSourceOwnerProgramLinks(row.source),program=decodeTypedProgramLinks(packet.network,packet.root),selected=emitRust(program,{sourceHeaderModel:fixture.model});
 const result=selected.definitions.find(text=>text.startsWith('pub fn result('));
 const bindings=[...result.matchAll(/let (ml_source_argument\w*) = /gu)].map(match=>({name:match[1],at:match.index}));
 assert.equal(bindings.length,2);assert.notEqual(bindings[0].name,bindings[1].name);
 const call='crate::leaf(&('+bindings[0].name+'), &('+bindings[1].name+'))';assert.equal(result.split(call).length-1,1);assert.ok(result.indexOf(call)>bindings[1].at);
 const hygiene=fixture.cases.find(row=>row.name.startsWith('fresh call names')),archive=createRustSourceOwnerProgramLinks(hygiene.source),output=emitRust(decodeTypedProgramLinks(archive.network,archive.root),{sourceHeaderModel:fixture.model}).definitions.find(text=>text.startsWith('pub fn caller('));
 assert.ok(!output.includes('let ml_source_argument = '));
});
