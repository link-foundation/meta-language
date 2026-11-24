// Deferred complete source products and actual retained predicate interfaces.
import assert from 'node:assert/strict';
import test from 'node:test';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {createRustSourceOwnerProgramLinks} from '../src/rust-source-owner-program.js';
import {decodeTypedProgramLinks,emitTypedProgramLinks,encodeTypedProgramLinks,TYPED_PROGRAM_LINKS_SCHEMA} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {emitRust} from '../src/translation/emit-rust.js';
import * as rules from '../src/translation/rust-string-predicate-interface-rules.js';
const fixture=readProjectData(new URL('../../parity/fixtures/rust-string-predicate-interface.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
const options={rustSourceHeaderEmissionModel:fixture.model};
function views(packet){return [packet.network,LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))];}
function value(packet){const emission=emitTypedProgramLinks(packet.network,packet.root,'JavaScript'),name=emission.mappings.find(mapping=>mapping.kind==='function'&&mapping.source==='result').target;return String(runInNewContext(emission.code.replace(/^export /gmu,'')+'\n'+name+'()',{}, {timeout:1000}));}
for(const row of fixture.cases)test(row.name,()=>{
 const packet=createRustSourceOwnerProgramLinks(row.source),program=decodeTypedProgramLinks(packet.network,packet.root),before=packet.network.toLino(),expected=emitTypedProgramLinks(packet.network,packet.root,'Rust',options);
 assert.ok(expected.encodings.some(row=>row.id==='retained-string-predicate-source-interface'));
 for(const network of views(packet)){
  assert.deepEqual([...decodeTypedProgramLinks(network,packet.root).declarations].map(([name,entry])=>[name,entry.sourceSignature]),[...program.declarations].map(([name,entry])=>[name,entry.sourceSignature]));
  assert.deepEqual(emitTypedProgramLinks(network,packet.root,'Rust',options),expected);
  for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));
 }
 assert.throws(()=>emitRust(program,{sourceHeaderModel:'retained textual Rust function interface 1'}),/additional source interface interpretation/u);
 assert.equal(value(packet),row.expected);assert.equal(packet.network.toLino(),before);
});
test('every shared predicate expression field and operand decision is independent',()=>{
 for(const row of fixture.selections)assert.equal(rules.acceptRustStringPredicateInterfaceModel(row.model),row.expected);
 for(const row of fixture.expressions)assert.equal(rules.acceptRustStringPredicateExpression(row.kind),row.expected);
 for(const row of fixture.fields)assert.equal(rules.acceptRustStringPredicateExpressionField(row.kind,row.field),row.expected);
 for(const row of fixture.operands)assert.equal(rules.acceptRustStringPredicateOperands(row.operation,row.result,row.object,row.search),row.expected);
});
for(const row of fixture.interfaceRefusals)test(row.name,()=>{
 const packet=createRustSourceOwnerProgramLinks(row.source),program=decodeTypedProgramLinks(packet.network,packet.root);
 assert.throws(()=>emitRust(program,{sourceHeaderModel:fixture.model}),error=>error.reason.includes(row.reason));
});
test('source-free predicate edits cannot change types operations operands or fields',()=>{
 const packet=createRustSourceOwnerProgramLinks(fixture.cases[0].source);
 for(const row of fixture.edits){const program=decodeTypedProgramLinks(packet.network,packet.root),node=program.declarations.get('predicate').body;assert.equal(node.k,'stringTest');let target=node;for(const field of row.path.slice(0,-1))target=target[field];target[row.path.at(-1)]=structuredClone(row.value);assert.throws(()=>{const linked=encodeTypedProgramLinks(program);emitTypedProgramLinks(linked.network,linked.root,'Rust',options);});
  if(row.path[0]==='position'){
   const network=LinkNetwork.fromLino(packet.network.toLino()),label=TYPED_PROGRAM_LINKS_SCHEMA.fields.k??'k',parent=network.fieldRelations().find(relation=>relation.label===label&&network.link(relation.child).metadata().definition==='stringTest').parent;
   const zero=network.links().find(link=>link.metadata().term===TYPED_PROGRAM_LINKS_SCHEMA.scalars.number&&link.metadata().definition===String(row.value)).id();
   network.insertFieldRelation(parent,'position',zero);for(const edited of views({network,root:packet.root}))assert.throws(()=>decodeTypedProgramLinks(edited,packet.root));
  }
 }
 const program=decodeTypedProgramLinks(packet.network,packet.root);program.utf16Strings=true;assert.throws(()=>{const linked=encodeTypedProgramLinks(program);emitTypedProgramLinks(linked.network,linked.root,'Rust',options);});
});
test('actual emitted predicate definitions reparse without original-source restoration',()=>{
 const original=createRustSourceOwnerProgramLinks(fixture.roundTripSource),program=decodeTypedProgramLinks(original.network,original.root),emission=emitRust(program,{sourceHeaderModel:fixture.model,arrayConsumption:true}),source=emission.definitions.join('\n\n')+'\nfn main(){}';
 assert.ok(source.includes('::std::borrow::ToOwned::to_owned'));assert.ok(source.includes('::std::string::String::as_str'));assert.ok(source.includes('.contains('));
 const restored=createRustSourceOwnerProgramLinks(source),signatures=packet=>[...decodeTypedProgramLinks(packet.network,packet.root).declarations].map(([name,entry])=>[name,entry.sourceSignature]);
 assert.deepEqual(signatures(restored),signatures(original));
 for(const packet of [original,restored]){assert.equal(value(packet),fixture.roundTripValue);for(const network of views(packet))for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));}
});
