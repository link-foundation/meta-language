// Deferred primitive Display products and actual retained source interfaces.
import assert from 'node:assert/strict';
import test from 'node:test';
import {runInNewContext} from 'node:vm';
import {readProjectData} from '../scripts/project-data.mjs';
import {createRustSourceOwnerProgramLinks} from '../src/rust-source-owner-program.js';
import {decodeTypedProgramLinks,emitTypedProgramLinks,encodeTypedProgramLinks,TYPED_PROGRAM_LINKS_SCHEMA} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {LinkMetadata,LinkType} from '../src/primitives.js';
import {acceptRustStandardStringOwnedMethodResult} from '../src/translation/rust-standard-owned-call-rules.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
import {emitRust} from '../src/translation/emit-rust.js';
import * as rules from '../src/translation/rust-displayed-string-interface-rules.js';
const fixture=readProjectData(new URL('../../parity/fixtures/rust-displayed-string-interface.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
const options={rustSourceHeaderEmissionModel:fixture.model};
test('each display selector expression field and complete operand has its premises',()=>{
 for(const row of fixture.selections)assert.equal(rules.acceptRustDisplayedStringInterfaceModel(row.model),row.expected);
 for(const row of fixture.expressions)assert.equal(rules.acceptRustDisplayedStringExpression(row.kind),row.expected);
 for(const row of fixture.fields)assert.equal(rules.acceptRustDisplayedStringExpressionField(row.kind,row.field),row.expected);
 for(const row of fixture.operands)assert.equal(rules.acceptRustPrimitiveDisplayOperand(row.kind,row.result,row.console),row.expected);
 for(const row of fixture.ownedMethodResults)assert.equal(acceptRustStandardStringOwnedMethodResult(row.method,row.kind,row.model,row.layers,row.proven,row.references,row.owner,row.path,row.children),row.expected);
});
test('the complete preceding Boolean input retains its original refusal interpretation',()=>{
 const row=fixture.previousInterpretation,packet=createRustSourceOwnerProgramLinks(row.source),program=decodeTypedProgramLinks(packet.network,packet.root);
 assert.throws(()=>emitRust(program,{sourceHeaderModel:row.model}),error=>error.reason.includes(row.reason));
 const source=emitTypedProgramLinks(packet.network,packet.root,'Rust',options);assert.ok(source.code.includes('.to_string()'));
});
test('repeated formatting fields retain exactly one complete argument call',()=>{
 const row=fixture.cases.find(row=>row.name.startsWith('repeated formatting')),packet=createRustSourceOwnerProgramLinks(row.source),body=decodeTypedProgramLinks(packet.network,packet.root).declarations.get('render').body;
 const seen=new WeakSet(),pending=[body];let calls=0;
 while(pending.length){const node=pending.pop();if(!node||typeof node!=='object'||seen.has(node))continue;seen.add(node);if(node.k==='call'&&node.fn==='input')calls+=1;for(const [field,child] of Object.entries(node))if(!['span','type','domain'].includes(field)){if(Array.isArray(child))pending.push(...child);else if(child&&typeof child==='object')pending.push(child);}}
 assert.equal(calls,1);
});
function views(packet){return [packet.network,LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))];}
function value(packet){const emission=emitTypedProgramLinks(packet.network,packet.root,'JavaScript'),name=emission.mappings.find(mapping=>mapping.kind==='function'&&mapping.source==='result').target;return String(runInNewContext(emission.code.replace(/^export /gmu,'')+'\n'+name+'()',{}, {timeout:1000}));}
for(const row of fixture.cases)test(row.name,()=>{
 const packet=createRustSourceOwnerProgramLinks(row.source),program=decodeTypedProgramLinks(packet.network,packet.root),before=packet.network.toLino(),expected=emitTypedProgramLinks(packet.network,packet.root,'Rust',options);
 assert.ok(expected.encodings.some(row=>row.id==='retained-displayed-string-source-interface'));
 for(const network of views(packet)){
  assert.deepEqual([...decodeTypedProgramLinks(network,packet.root).declarations].map(([name,entry])=>[name,entry.sourceSignature]),[...program.declarations].map(([name,entry])=>[name,entry.sourceSignature]));
  assert.deepEqual(emitTypedProgramLinks(network,packet.root,'Rust',options),expected);
  for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));
 }
 assert.throws(()=>emitRust(program,{sourceHeaderModel:'retained constructed Rust string function interface 1'}),/additional source interface interpretation/u);
 assert.equal(value(packet),row.expected);assert.equal(packet.network.toLino(),before);
});
for(const row of fixture.interfaceRefusals)test(row.name,()=>{
 const packet=createRustSourceOwnerProgramLinks(row.source),program=decodeTypedProgramLinks(packet.network,packet.root);
 assert.throws(()=>emitRust(program,{sourceHeaderModel:fixture.model}),error=>error.reason.includes(row.reason));
});
test('source-free primitive display edits cannot change types operations operands or fields',()=>{
 const packet=createRustSourceOwnerProgramLinks(fixture.cases[0].source);
 for(const row of fixture.edits){const program=decodeTypedProgramLinks(packet.network,packet.root);let node=program.declarations.get('render').body;while(node.k==='let')node=node.body;assert.equal(node.k,'toString');let target=node;for(const field of row.path.slice(0,-1))target=target[field];target[row.path.at(-1)]=structuredClone(row.value);assert.throws(()=>{const linked=encodeTypedProgramLinks(program);emitTypedProgramLinks(linked.network,linked.root,'Rust',options);});
  if(row.path[0]==='position'){
   const network=LinkNetwork.fromLino(packet.network.toLino()),label=TYPED_PROGRAM_LINKS_SCHEMA.fields.k??'k',parent=network.fieldRelations().find(relation=>relation.label===label&&network.link(relation.child).metadata().definition==='toString').parent;
   const zero=network.insertLink([],LinkMetadata.new().withLinkType(LinkType.Semantic).withNamed(true).withLanguage(TYPED_PROGRAM_LINKS_SCHEMA.language).withTerm(TYPED_PROGRAM_LINKS_SCHEMA.scalars.number).withDefinition(String(row.value)));
   network.insertFieldRelation(parent,'position',zero);for(const edited of views({network,root:packet.root}))assert.throws(()=>decodeTypedProgramLinks(edited,packet.root));
  }
 }
 const program=decodeTypedProgramLinks(packet.network,packet.root);program.utf16Strings=true;assert.throws(()=>{const linked=encodeTypedProgramLinks(program);emitTypedProgramLinks(linked.network,linked.root,'Rust',options);});
});
test('actual emitted primitive display definitions reparse without original-source restoration',()=>{
 const original=createRustSourceOwnerProgramLinks(fixture.roundTripSource),program=decodeTypedProgramLinks(original.network,original.root),emission=emitRust(program,{sourceHeaderModel:fixture.model,arrayConsumption:true}),source=emission.definitions.join('\n\n')+'\nfn main(){}';
 assert.ok(source.includes('::std::borrow::ToOwned::to_owned'));assert.ok(source.includes('::std::string::String::as_str'));assert.ok(source.includes('format!'));
 const restored=createRustSourceOwnerProgramLinks(source),signatures=packet=>[...decodeTypedProgramLinks(packet.network,packet.root).declarations].map(([name,entry])=>[name,entry.sourceSignature]);
 assert.deepEqual(signatures(restored),signatures(original));
 for(const packet of [original,restored]){assert.equal(value(packet),fixture.roundTripValue);for(const network of views(packet))for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(network,packet.root,target),emitTypedProgramLinks(packet.network,packet.root,target));}
});
