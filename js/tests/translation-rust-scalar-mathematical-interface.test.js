// Deferred complete interfaces and the exact independently selected old domain.
import assert from 'node:assert/strict';
import test from 'node:test';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {createRustSourceOwnerProgramLinks} from '../src/rust-source-owner-program.js';
import {decodeTypedProgramLinks,emitTypedProgramLinks,encodeTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {emitRust} from '../src/translation/emit-rust.js';
import * as rules from '../src/translation/rust-scalar-mathematical-interface-rules.js';
const fixture=readProjectData(new URL('../../parity/fixtures/rust-scalar-mathematical-interface.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
const options={rustSourceHeaderEmissionModel:fixture.model};
function views(packet){return [packet.network,LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))];}
for(const row of fixture.cases)test(row.name,()=>{
 const packet=createRustSourceOwnerProgramLinks(row.source),program=decodeTypedProgramLinks(packet.network,packet.root),before=packet.network.toLino();
 const expected=emitTypedProgramLinks(packet.network,packet.root,'Rust',options);assert.ok(expected.encodings.some(row=>row.id==='retained-mathematical-source-interface'));
 for(const network of views(packet)){
  assert.deepEqual([...decodeTypedProgramLinks(network,packet.root).declarations].map(([name,entry])=>[name,entry.sourceSignature]),[...program.declarations].map(([name,entry])=>[name,entry.sourceSignature]));
  assert.deepEqual(emitTypedProgramLinks(network,packet.root,'Rust',options),expected);
  for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));
 }
 assert.equal(packet.network.toLino(),before);
 assert.throws(()=>emitRust(program,{sourceHeaderModel:'retained scalar Rust function interface 1'}),/additional source interface interpretation/u);
 const javascript=emitTypedProgramLinks(packet.network,packet.root,'JavaScript'),name=javascript.mappings.find(mapping=>mapping.kind==='function'&&mapping.source==='result').target;
 const value=runInNewContext(javascript.code.replace(/^export /gmu,'')+'\n'+name+'()',{}, {timeout:1000});assert.equal(String(value),row.expected);
 assert.throws(()=>emitTypedProgramLinks(packet.network,packet.root,'JavaScript',options),/requires Rust target/u);
});
test('every selection expression field and mathematical operation has an independent decision',()=>{
 for(const row of fixture.selections)assert.equal(rules.acceptRustScalarMathematicalInterfaceModel(row.model),row.expected);
 for(const row of fixture.expressions)assert.equal(rules.acceptRustScalarMathematicalExpression(row.kind),row.expected);
 for(const row of fixture.fields)assert.equal(rules.acceptRustScalarMathematicalExpressionField(row.kind,row.field),row.expected);
 for(const row of fixture.operations)assert.equal(rules.acceptRustScalarMathematicalOperation(row.operation,row.count,row.result),row.expected);
});
test('source-free edits retain the extra interface boundary and raw scalar fields',()=>{
 const row=fixture.cases.find(row=>row.name.startsWith('binary64 abs')),packet=createRustSourceOwnerProgramLinks(row.source);
 const program=decodeTypedProgramLinks(packet.network,packet.root),expression=program.declarations.get('apply').body;assert.equal(expression.k,'math');expression.op='sqrt';
 assert.throws(()=>emitRust(program,{sourceHeaderModel:fixture.model}),/admitted scalar result/u);
 expression.op='abs';expression.unrepresented=true;assert.throws(()=>emitRust(program,{sourceHeaderModel:fixture.model}),/additional source interface interpretation/u);
 const borrowed=fixture.cases.find(row=>row.name.startsWith('immutable-reference')),source=createRustSourceOwnerProgramLinks(borrowed.source),edited=decodeTypedProgramLinks(source.network,source.root);edited.declarations.get('combine').sourceSignature.parameters[0].lifetimes=['static'];
 const linked=encodeTypedProgramLinks(edited);for(const network of views(linked))assert.throws(()=>emitTypedProgramLinks(network,linked.root,'Rust',options),/nonstatic copied borrows/u);
});
